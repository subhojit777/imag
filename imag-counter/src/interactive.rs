use std::collections::BTreeMap;
use std::fmt::{Display, Formatter, Error};
use std::io::Write;
use std::io::stderr;
use std::io::stdin;
use std::process::exit;
use std::result::Result as RResult;

use libimagcounter::counter::Counter;
use libimagcounter::error::CounterError;
use libimagrt::runtime::Runtime;
use libimagutil::key_value_split::IntoKeyValue;
use libimagerror::trace::trace_error;

type Result<T> = RResult<T, CounterError>;

pub fn interactive(rt: &Runtime) {
    let scmd = rt.cli().subcommand_matches("interactive");
    if scmd.is_none() {
        debug!("No subcommand");
        exit(1);
    }
    let scmd = scmd.unwrap();
    debug!("Found 'interactive' command");

    let mut pairs : BTreeMap<char, Binding> = BTreeMap::new();

    for spec in scmd.values_of("spec").unwrap() {
        match compute_pair(rt, &spec) {
            Ok((k, v)) => { pairs.insert(k, v); },
            Err(e) => { trace_error(&e); },
        }
    }

    if !has_quit_binding(&pairs) {
        pairs.insert('q', Binding::Function(String::from("quit"), Box::new(quit)));
    }

    stderr().flush().ok();
    loop {
        println!("---");
        for (k, v) in &pairs {
            println!("\t[{}] => {}", k, v);
        }
        println!("---");
        print!("counter > ");

        let mut input = String::new();
        if let Err(e) = stdin().read_line(&mut input) {
            trace_error(&e);
            exit(1);
        }

        let cont = if !input.is_empty() {
            let increment = match input.chars().next() { Some('-') => false, _ => true };
            input.chars().all(|chr| {
                match pairs.get_mut(&chr) {
                    Some(&mut Binding::Counter(ref mut ctr)) => {
                        if increment {
                            debug!("Incrementing");
                            if let Err(e) = ctr.inc() {
                                trace_error(&e);
                            }
                        } else {
                            debug!("Decrementing");
                            if let Err(e) = ctr.dec() {
                                trace_error(&e);
                            }
                        }
                        true
                    },
                    Some(&mut Binding::Function(ref name, ref f))   => {
                        debug!("Calling {}", name);
                        f()
                    },
                    None => true,
                }
            })
        } else {
            println!("No input...");
            println!("\tUse a single character to increment the counter which is bound to it");
            println!("\tUse 'q' (or the character bound to quit()) to exit");
            println!("\tPrefix the line with '-' to decrement instead of increment the counters");
            println!("");
            true
        };

        if !cont {
            break;
        }
    }
}

fn has_quit_binding(pairs: &BTreeMap<char, Binding>) -> bool {
    pairs.iter()
        .any(|(_, bind)| {
            match *bind {
                Binding::Function(ref name, _) => name == "quit",
                _ => false,
            }
        })
}

enum Binding<'a> {
    Counter(Counter<'a>),
    Function(String, Box<Fn() -> bool>),
}

impl<'a> Display for Binding<'a> {

    fn fmt(&self, fmt: &mut Formatter) -> RResult<(), Error> {
        match *self {
            Binding::Counter(ref c) => {
                match c.name() {
                    Ok(name) => {
                        try!(write!(fmt, "{}", name));
                        Ok(())
                    },
                    Err(e) => {
                        trace_error(&e);
                        Ok(()) // TODO: Find a better way to escalate here.
                    },
                }
            },
            Binding::Function(ref name, _) => write!(fmt, "{}()", name),
        }
    }

}

fn compute_pair<'a>(rt: &'a Runtime, spec: &str) -> Result<(char, Binding<'a>)> {
    let kv = String::from(spec).into_kv();
    if kv.is_none() {
        debug!("Key-Value parsing failed!");
        exit(1);
    }
    let kv = kv.unwrap();

    let (k, v) = kv.into();
    if !k.len() == 1 {
        // We have a key which is not only a single character!
        exit(1);
    }

    if v == "quit" {
        // TODO uncaught unwrap()
        Ok((k.chars().next().unwrap(), Binding::Function(String::from("quit"), Box::new(quit))))
    } else {
        // TODO uncaught unwrap()
        Counter::load(v, rt.store()).and_then(|ctr| Ok((k.chars().next().unwrap(), Binding::Counter(ctr))))
    }
}

fn quit() -> bool {
    false
}

