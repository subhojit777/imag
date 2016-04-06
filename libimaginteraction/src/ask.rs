// functions to ask the user for data, with crate:spinner

use std::io::stdin;

use regex::Regex;
use ansi_term::Colour::*;

/// Ask the user for a Yes/No answer. Optionally provide a default value. If none is provided, this
/// keeps loop{}ing
pub fn ask_bool(s: &str, default: Option<bool>) -> bool {
    lazy_static! {
        static ref R_YES: Regex = Regex::new(r"^[Yy]$").unwrap();
        static ref R_NO: Regex  = Regex::new(r"^[Nn]$").unwrap();
    }

    loop {
        ask_question(s, false);
        if match default { Some(s) => s, _ => true } {
            println!(" [Yn]: ");
        } else {
            println!(" [yN]: ");
        }

        let mut s = String::new();
        let _     = stdin().read_line(&mut s);

        if R_YES.is_match(&s[..]) {
            return true
        } else if R_NO.is_match(&s[..]) {
            return false
        } else {
            if default.is_some() {
                return default.unwrap();
            }

            // else again...
        }
    }
}

pub fn ask_uint(s: &str) -> u64 {
    unimplemented!()
}

pub fn ask_string(s: &str) -> String {
    unimplemented!()
}

pub fn ask_enum<E: From<String>>(s: &str) -> E {
    unimplemented!()
}

/// Helper function to print a imag question string. The `question` argument may not contain a
/// trailing questionmark.
///
/// The `nl` parameter can be used to configure whether a newline character should be printed
pub fn ask_question(question: &str, nl: bool) {
    if nl {
        println!("[imag]: {}?", Yellow.paint(question));
    } else {
        print!("[imag]: {}?", Yellow.paint(question));
    }
}

