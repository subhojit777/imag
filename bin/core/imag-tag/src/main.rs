//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015-2018 Matthias Beyer <mail@beyermatthias.de> and contributors
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; version
// 2.1 of the License.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
//

extern crate clap;
#[macro_use] extern crate log;

#[cfg(test)] extern crate toml;

extern crate libimagstore;
extern crate libimagrt;
extern crate libimagentrytag;
extern crate libimagerror;

#[cfg(test)]
#[macro_use]
extern crate libimagutil;

#[cfg(not(test))]
extern crate libimagutil;

#[cfg(test)]
extern crate toml_query;

#[cfg(test)]
extern crate env_logger;

use std::path::PathBuf;

use libimagrt::runtime::Runtime;
use libimagrt::setup::generate_runtime_setup;
use libimagentrytag::tagable::Tagable;
use libimagentrytag::tag::Tag;
use libimagerror::trace::{trace_error, trace_error_exit};
use libimagstore::storeid::StoreId;
use libimagutil::warn_exit::warn_exit;

use clap::ArgMatches;

mod ui;

use ui::build_ui;

fn main() {
    let rt = generate_runtime_setup("imag-store",
                                    env!("CARGO_PKG_VERSION"),
                                    "Direct interface to the store. Use with great care!",
                                    build_ui);

    let id = rt.cli().value_of("id").unwrap(); // enforced by clap
    rt.cli()
        .subcommand_name()
        .map_or_else(
            || {
                let id = PathBuf::from(id);
                let add = get_add_tags(rt.cli());
                let rem = get_remove_tags(rt.cli());
                alter(&rt, id, add, rem);
            },
            |name| {
                let id = PathBuf::from(id);
                debug!("Call: {}", name);
                match name {
                    "list" => list(id, &rt),
                    _ => {
                        warn!("Unknown command");
                        // More error handling
                    },
                };
            });
}

fn alter(rt: &Runtime, id: PathBuf, add: Option<Vec<Tag>>, rem: Option<Vec<Tag>>) {
    let path = {
        match StoreId::new(Some(rt.store().path().clone()), id) {
            Err(e) => trace_error_exit(&e, 1),
            Ok(s) => s,
        }
    };
    debug!("path = {:?}", path);

    match rt.store().get(path) {
        Ok(Some(mut e)) => {
            debug!("Entry header now = {:?}", e.get_header());

            add.map(|tags| {
                    debug!("Adding tags = '{:?}'", tags);
                    for tag in tags {
                        debug!("Adding tag '{:?}'", tag);
                        if let Err(e) = e.add_tag(tag) {
                            trace_error(&e);
                        } else {
                            debug!("Adding tag worked");
                        }
                    }
                }); // it is okay to ignore a None here

            debug!("Entry header now = {:?}", e.get_header());

            rem.map(|tags| {
                debug!("Removing tags = '{:?}'", tags);
                for tag in tags {
                    debug!("Removing tag '{:?}'", tag);
                    if let Err(e) = e.remove_tag(tag) {
                        trace_error(&e);
                    }
                }
            }); // it is okay to ignore a None here

            debug!("Entry header now = {:?}", e.get_header());

        },

        Ok(None) => {
            info!("No entry found.");
        },

        Err(e) => {
            info!("No entry.");
            trace_error(&e);
        },
    }
}

fn list(id: PathBuf, rt: &Runtime) {
    let path = match StoreId::new(Some(rt.store().path().clone()), id) {
        Err(e) => trace_error_exit(&e, 1),
        Ok(s)  => s,
    };
    debug!("path = {:?}", path);

    let entry = match rt.store().get(path.clone()) {
        Ok(Some(e)) => e,
        Ok(None) => warn_exit("No entry found.", 1),

        Err(e) => {
            warn!("Could not get entry '{:?}'", path);
            trace_error_exit(&e, 1);
        },
    };

    let scmd = rt.cli().subcommand_matches("list").unwrap(); // safe, we checked in main()

    let json_out = scmd.is_present("json");
    let line_out = scmd.is_present("linewise");
    let sepp_out = scmd.is_present("sep");
    let mut comm_out = scmd.is_present("commasep");

    if !vec![json_out, line_out, comm_out, sepp_out].iter().any(|v| *v) {
        // None of the flags passed, go to default
        comm_out = true;
    }

    let tags = entry.get_tags();
    if tags.is_err() {
        trace_error_exit(&tags.unwrap_err(), 1);
    }
    let tags = tags.unwrap();

    if json_out {
        unimplemented!()
    }

    if line_out {
        for tag in &tags {
            println!("{}", tag);
        }
    }

    if sepp_out {
        let sepp = scmd.value_of("sep").unwrap(); // we checked before
        println!("{}", tags.join(sepp));
    }

    if comm_out {
        println!("{}", tags.join(", "));
    }
}

/// Get the tags which should be added from the commandline
///
/// Returns none if the argument was not specified
fn get_add_tags(matches: &ArgMatches) -> Option<Vec<Tag>> {
    let a = "add-tags";
    extract_tags(matches, a, '+')
        .or_else(|| matches.values_of(a).map(|values| values.map(String::from).collect()))
}

/// Get the tags which should be removed from the commandline
///
/// Returns none if the argument was not specified
fn get_remove_tags(matches: &ArgMatches) -> Option<Vec<Tag>> {
    let r = "remove-tags";
    extract_tags(matches, r, '+')
        .or_else(|| matches.values_of(r).map(|values| values.map(String::from).collect()))
}

fn extract_tags(matches: &ArgMatches, specifier: &str, specchar: char) -> Option<Vec<Tag>> {
    if let Some(submatch) = matches.subcommand_matches("tags") {
        submatch.values_of(specifier)
            .map(|values| values.map(String::from).collect())
    } else {
        matches.values_of("specify-tags")
            .map(|argmatches| {
                argmatches
                    .map(String::from)
                    .filter(|s| s.starts_with(specchar))
                    .map(|s| {
                        String::from(s.split_at(1).1)
                    })
                    .collect()
            })
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::ffi::OsStr;

    use toml::value::Value;
    use toml_query::read::TomlValueReadExt;
    use toml_query::error::Result as TomlQueryResult;

    use libimagrt::runtime::Runtime;
    use libimagstore::storeid::StoreId;
    use libimagstore::store::{Result as StoreResult, FileLockEntry, Entry};

    use super::*;

    make_mock_app! {
        app "imag-tag";
        modulename mock;
        version "0.6.3";
        with help "imag-tag mocking app";
    }
    use self::mock::generate_test_runtime;

    fn create_test_default_entry<'a, S: AsRef<OsStr>>(rt: &'a Runtime, name: S) -> StoreResult<StoreId> {
        let mut path = PathBuf::new();
        path.set_file_name(name);

        let default_entry = Entry::new(StoreId::new_baseless(PathBuf::from("")).unwrap()).to_str();

        let id = StoreId::new_baseless(path)?;
        let mut entry = rt.store().create(id.clone())?;
        entry.get_content_mut().push_str(&default_entry);

        Ok(id)
    }

    fn get_entry_tags<'a>(entry: &'a FileLockEntry<'a>) -> TomlQueryResult<Option<&'a Value>> {
        entry.get_header().read(&"tag.values".to_owned())
    }

    fn tags_toml_value<'a, I: IntoIterator<Item = &'static str>>(tags: I) -> Value {
        Value::Array(tags.into_iter().map(|s| Value::String(s.to_owned())).collect())
    }

    fn setup_logging() {
        let _ = ::env_logger::try_init();
    }

    #[test]
    fn test_tag_add_adds_tag() {
        setup_logging();
        debug!("Generating runtime");
        let name = "test-tag-add-adds-tags";
        let rt = generate_test_runtime(vec![name, "--add", "foo"]).unwrap();

        debug!("Creating default entry");
        create_test_default_entry(&rt, name).unwrap();
        let id = PathBuf::from(String::from(name));

        debug!("Getting 'add' tags");
        let add = get_add_tags(rt.cli());
        debug!("Add-tags: {:?}", add);

        debug!("Getting 'remove' tags");
        let rem = get_remove_tags(rt.cli());
        debug!("Rem-tags: {:?}", rem);

        debug!("Altering things");
        alter(&rt, id.clone(), add, rem);
        debug!("Altered");

        let test_entry = rt.store().get(id).unwrap().unwrap();

        let test_tags  = get_entry_tags(&test_entry);
        assert!(test_tags.is_ok(), "Should be Ok(_) = {:?}", test_tags);

        let test_tags  = test_tags.unwrap();
        assert!(test_tags.is_some(), "Should be Some(_) = {:?}", test_tags);
        let test_tags  = test_tags.unwrap();

        assert_ne!(*test_tags, tags_toml_value(vec![]));
        assert_eq!(*test_tags, tags_toml_value(vec!["foo"]));
    }

    #[test]
    fn test_tag_add_more_than_remove_adds_tags() {
        setup_logging();
        debug!("Generating runtime");
        let name = "test-tag-add-more-than-remove-adds-tags";
        let rt = generate_test_runtime(vec![name,
                                       "--add", "foo",
                                       "--add", "bar",
                                       "--add", "baz",
                                       "--add", "bub",
                                       "--remove", "foo",
                                       "--remove", "bar",
                                       "--remove", "baz",
        ]).unwrap();

        debug!("Creating default entry");
        create_test_default_entry(&rt, name).unwrap();
        let id = PathBuf::from(String::from(name));

        // Manually add tags
        let add = get_add_tags(rt.cli());

        debug!("Getting 'remove' tags");
        let rem = get_remove_tags(rt.cli());
        debug!("Rem-tags: {:?}", rem);

        debug!("Altering things");
        alter(&rt, id.clone(), add, rem);
        debug!("Altered");

        let test_entry = rt.store().get(id).unwrap().unwrap();
        let test_tags  = get_entry_tags(&test_entry).unwrap().unwrap();

        assert_eq!(*test_tags, tags_toml_value(vec!["bub"]));
    }

    #[test]
    fn test_tag_remove_removes_tag() {
        setup_logging();
        debug!("Generating runtime");
        let name = "test-tag-remove-removes-tag";
        let rt = generate_test_runtime(vec![name, "--remove", "foo"]).unwrap();

        debug!("Creating default entry");
        create_test_default_entry(&rt, name).unwrap();
        let id = PathBuf::from(String::from(name));

        // Manually add tags
        let add = Some(vec![ "foo".to_owned() ]);

        debug!("Getting 'remove' tags");
        let rem = get_remove_tags(rt.cli());
        debug!("Rem-tags: {:?}", rem);

        debug!("Altering things");
        alter(&rt, id.clone(), add, rem);
        debug!("Altered");

        let test_entry = rt.store().get(id).unwrap().unwrap();
        let test_tags  = get_entry_tags(&test_entry).unwrap().unwrap();

        assert_eq!(*test_tags, tags_toml_value(vec![]));
    }

    #[test]
    fn test_tag_remove_removes_only_to_remove_tag() {
        setup_logging();
        debug!("Generating runtime");
        let name = "test-tag-remove-removes-only-to-remove-tag-doesnt-crash-on-nonexistent-tag";
        let rt = generate_test_runtime(vec![name, "--remove", "foo"]).unwrap();

        debug!("Creating default entry");
        create_test_default_entry(&rt, name).unwrap();
        let id = PathBuf::from(String::from(name));

        // Manually add tags
        let add = Some(vec![ "foo".to_owned(), "bar".to_owned() ]);

        debug!("Getting 'remove' tags");
        let rem = get_remove_tags(rt.cli());
        debug!("Rem-tags: {:?}", rem);

        debug!("Altering things");
        alter(&rt, id.clone(), add, rem);
        debug!("Altered");

        let test_entry = rt.store().get(id).unwrap().unwrap();
        let test_tags  = get_entry_tags(&test_entry).unwrap().unwrap();

        assert_eq!(*test_tags, tags_toml_value(vec!["bar"]));
    }

    #[test]
    fn test_tag_remove_removes_but_doesnt_crash_on_nonexistent_tag() {
        setup_logging();
        debug!("Generating runtime");
        let name = "test-tag-remove-removes-but-doesnt-crash-on-nonexistent-tag";
        let rt = generate_test_runtime(vec![name, "--remove", "foo", "--remove", "bar"]).unwrap();

        debug!("Creating default entry");
        create_test_default_entry(&rt, name).unwrap();
        let id = PathBuf::from(String::from(name));

        // Manually add tags
        let add = Some(vec![ "foo".to_owned() ]);

        debug!("Getting 'remove' tags");
        let rem = get_remove_tags(rt.cli());
        debug!("Rem-tags: {:?}", rem);

        debug!("Altering things");
        alter(&rt, id.clone(), add, rem);
        debug!("Altered");

        let test_entry = rt.store().get(id).unwrap().unwrap();
        let test_tags  = get_entry_tags(&test_entry).unwrap().unwrap();

        assert_eq!(*test_tags, tags_toml_value(vec![]));
    }

}

