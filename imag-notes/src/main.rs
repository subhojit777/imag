extern crate clap;
#[macro_use] extern crate log;
extern crate semver;
#[macro_use] extern crate version;

extern crate libimagnotes;
extern crate libimagrt;
extern crate libimagentrytag;
extern crate libimagutil;

use std::process::exit;

use libimagrt::edit::Edit;
use libimagrt::runtime::Runtime;
use libimagnotes::note::Note;
use libimagutil::trace::trace_error;

mod ui;
use ui::build_ui;

fn main() {
    let name = "imag-notes";
    let version = &version!()[..];
    let about = "Note taking helper";
    let ui = build_ui(Runtime::get_default_cli_builder(name, version, about));
    let rt = {
        let rt = Runtime::new(ui);
        if rt.is_ok() {
            rt.unwrap()
        } else {
            println!("Could not set up Runtime");
            println!("{:?}", rt.unwrap_err());
            exit(1);
        }
    };

    rt.cli()
        .subcommand_name()
        .map(|name| {
            debug!("Call: {}", name);
            match name {
                "create" => create(&rt),
                "delete" => delete(&rt),
                "edit"   => edit(&rt),
                "list"   => list(&rt),
                _        => {
                    debug!("Unknown command"); // More error handling
                },
            };
        });
}

fn name_from_cli(rt: &Runtime, subcmd: &str) -> String {
    rt.cli().subcommand_matches(subcmd).unwrap().value_of("name").map(String::from).unwrap()
}

fn create(rt: &Runtime) {
    let name = name_from_cli(rt, "create");
    Note::new(rt.store(), name.clone(), String::new())
        .map_err(|e| trace_error(&e))
        .ok();

    if rt.cli().subcommand_matches("create").unwrap().is_present("edit") {
        if !edit_entry(rt, name) {
            exit(1);
        }
    }
}

fn delete(rt: &Runtime) {
    Note::delete(rt.store(), String::from(name_from_cli(rt, "delete")))
        .map_err(|e| trace_error(&e))
        .map(|_| println!("Ok"))
        .ok();
}

fn edit(rt: &Runtime) {
    edit_entry(rt, name_from_cli(rt, "edit"));
}

fn edit_entry(rt: &Runtime, name: String) -> bool {
    let note = Note::retrieve(rt.store(), name);
    if note.is_err() {
        trace_error(&note.unwrap_err());
        warn!("Cannot edit nonexistent Note");
        return false
    }

    let mut note = note.unwrap();
    if let Err(e) = note.edit_content(rt) {
        trace_error(&e);
        warn!("Editing failed");
        return false
    }
    true
}

fn list(rt: &Runtime) {
    use std::cmp::Ordering;

    let iter = Note::all_notes(rt.store());
    if iter.is_err() {
        trace_error(&iter.unwrap_err());
        exit(1);
    }

    let mut iter = iter.unwrap()
        .filter_map(|note| {
            match note {
                Err(e) => {
                    trace_error(&e);
                    None
                },
                Ok(e) => Some(e)
            }
        })
        .collect::<Vec<Note>>();

    iter.sort_by(|note_a, note_b| {
        if let (Ok(a), Ok(b)) = (note_a.get_name(), note_b.get_name()) {
            return a.cmp(&b)
        } else {
            return Ordering::Greater;
        }
    });

    for note in iter {
        note.get_name()
            .map(|name| println!("{}", name))
            .map_err(|e| trace_error(&e))
            .ok();
    }
}

