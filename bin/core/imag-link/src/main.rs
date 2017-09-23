//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015, 2016 Matthias Beyer <mail@beyermatthias.de> and contributors
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

#![deny(
    non_camel_case_types,
    non_snake_case,
    path_statements,
    trivial_numeric_casts,
    unstable_features,
    unused_allocation,
    unused_import_braces,
    unused_imports,
    unused_must_use,
    unused_mut,
    unused_qualifications,
    while_true,
)]

#[macro_use] extern crate log;
extern crate clap;
extern crate url;
#[macro_use] extern crate version;
#[cfg(test)] extern crate toml;
#[cfg(test)] extern crate toml_query;

extern crate libimagentrylink;
extern crate libimagrt;
extern crate libimagstore;
extern crate libimagerror;

#[cfg(test)]
#[macro_use]
extern crate libimagutil;

#[cfg(not(test))]
extern crate libimagutil;

use std::ops::Deref;

use libimagrt::runtime::Runtime;
use libimagrt::setup::generate_runtime_setup;
use libimagstore::error::StoreError;
use libimagstore::store::Entry;
use libimagstore::store::FileLockEntry;
use libimagstore::store::Store;
use libimagerror::trace::{MapErrTrace, trace_error, trace_error_exit};
use libimagentrylink::external::ExternalLinker;
use libimagentrylink::internal::InternalLinker;
use libimagutil::warn_result::*;
use libimagutil::warn_exit::warn_exit;
use libimagutil::info_result::*;
use clap::ArgMatches;
use url::Url;

mod ui;

use ui::build_ui;

fn main() {
    let rt = generate_runtime_setup("imag-link",
                                    &version!()[..],
                                    "Link entries",
                                    build_ui);

    rt.cli()
        .subcommand_name()
        .map(|name| {
            match name {
                "internal" => handle_internal_linking(&rt),
                "external" => handle_external_linking(&rt),
                _ => warn_exit("No commandline call", 1)
            }
        });
}

fn handle_internal_linking(rt: &Runtime) {
    use libimagentrylink::internal::store_check::StoreLinkConsistentExt;

    debug!("Handle internal linking call");
    let cmd = rt.cli().subcommand_matches("internal").unwrap();

    if cmd.is_present("check-consistency") {
        match rt.store().check_link_consistency() {
            Ok(_) => {
                info!("Store is consistent");
                return;
            }
            Err(e) => {
                trace_error(&e);
                ::std::process::exit(1);
            }
        }
    }

    match cmd.subcommand_name() {
        Some("list") => {
            cmd.subcommand_matches("list")
                .map(|matches| handle_internal_linking_list_call(rt, cmd, matches));
        },
        Some("add") => {
            let (mut from, to) = get_from_to_entry(&rt, "add");
            for mut to_entry in to {
                if let Err(e) = to_entry.add_internal_link(&mut from) {
                    trace_error_exit(&e, 1);
                }
            };
        },

        Some("remove") => {
            let (mut from, to) = get_from_to_entry(&rt, "remove");
            for mut to_entry in to {
                if let Err(e) = to_entry.remove_internal_link(&mut from) {
                    trace_error_exit(&e, 1);
                }
            };
        },

        _ => unreachable!(),
    }
}

#[inline]
fn handle_internal_linking_list_call(rt: &Runtime, cmd: &ArgMatches, list: &ArgMatches) {
    use libimagentrylink::external::is_external_link_storeid;

    debug!("List...");
    for entry in list.values_of("entries").unwrap() { // clap has our back
        debug!("Listing for '{}'", entry);
        match get_entry_by_name(rt, entry) {
            Ok(Some(e)) => {
                e.get_internal_links()
                    .map(|iter| {
                        iter.filter(move |id| {
                            cmd.is_present("list-externals-too") || !is_external_link_storeid(&id)
                        })
                    })
                    .map(|links| {
                        let i = links
                            .filter_map(|l| {
                                l.to_str()
                                    .map_warn_err(|e| format!("Failed to convert StoreId to string: {:?}", e))
                                    .ok()
                            })
                            .enumerate();

                        for (i, link) in i {
                            println!("{: <3}: {}", i, link);
                        }
                    })
                    .map_err_trace()
                    .ok();
            },

            Ok(None) => {
                warn!("Entry not found: {:?}", entry);
                break;
            }

            Err(e) => {
                trace_error(&e);
                break;
            },
        }
    }
    debug!("Listing ready!");
}

fn get_from_to_entry<'a>(rt: &'a Runtime, subcommand: &str) -> (FileLockEntry<'a>, Vec<FileLockEntry<'a>>) {
    let from = match get_from_entry(&rt, subcommand) {
        None => warn_exit("No 'from' entry", 1),
        Some(s) => s,
    };
    debug!("Link from = {:?}", from.deref());

    let to = match get_to_entries(&rt, subcommand) {
        None => warn_exit("No 'to' entry", 1),
        Some(to) => to,
    };
    debug!("Link to = {:?}", to.iter().map(|f| f.deref()).collect::<Vec<&Entry>>());

    (from, to)
}

fn get_from_entry<'a>(rt: &'a Runtime, subcommand: &str) -> Option<FileLockEntry<'a>> {
    rt.cli()
        .subcommand_matches("internal")
        .unwrap() // safe, we know there is an "internal" subcommand"
        .subcommand_matches(subcommand)
        .unwrap() // safe, we know there is an "add" subcommand
        .value_of("from")
        .and_then(|from_name| {
            match get_entry_by_name(rt, from_name) {
                Err(e) => {
                    debug!("We couldn't get the entry from name: '{:?}'", from_name);
                    trace_error(&e); None
                },
                Ok(Some(e)) => Some(e),
                Ok(None)    => None,
            }

        })
}

fn get_to_entries<'a>(rt: &'a Runtime, subcommand: &str) -> Option<Vec<FileLockEntry<'a>>> {
    rt.cli()
        .subcommand_matches("internal")
        .unwrap() // safe, we know there is an "internal" subcommand"
        .subcommand_matches(subcommand)
        .unwrap() // safe, we know there is an "add" subcommand
        .values_of("to")
        .map(|values| {
            let mut v = vec![];
            for entry in values.map(|v| get_entry_by_name(rt, v)) {
                match entry {
                    Err(e) => trace_error(&e),
                    Ok(Some(e)) => v.push(e),
                    Ok(None) => warn!("Entry not found: {:?}", v),
                }
            }
            v
        })
}

fn get_entry_by_name<'a>(rt: &'a Runtime, name: &str) -> Result<Option<FileLockEntry<'a>>, StoreError> {
    use std::path::PathBuf;
    use libimagstore::storeid::StoreId;

    StoreId::new(Some(rt.store().path().clone()), PathBuf::from(name))
        .and_then(|id| rt.store().get(id))
}

fn handle_external_linking(rt: &Runtime) {
    let scmd       = rt.cli().subcommand_matches("external").unwrap();
    let entry_name = scmd.value_of("id").unwrap(); // enforced by clap
    let mut entry  = match get_entry_by_name(rt, entry_name) {
        Err(e) => trace_error_exit(&e, 1),
        Ok(None) => {
            warn!("Entry not found: {:?}", entry_name);
            return;
        },
        Ok(Some(entry)) => entry
    };

    if scmd.is_present("add") {
        debug!("Adding link to entry!");
        add_link_to_entry(rt.store(), scmd, &mut entry);
        return;
    }

    if scmd.is_present("remove") {
        debug!("Removing link from entry!");
        remove_link_from_entry(rt.store(), scmd, &mut entry);
        return;
    }

    if scmd.is_present("set") {
        debug!("Setting links in entry!");
        set_links_for_entry(rt.store(), scmd, &mut entry);
        return;
    }

    if scmd.is_present("list") {
        debug!("Listing links in entry!");
        list_links_for_entry(rt.store(), &mut entry);
        return;
    }

    panic!("Clap failed to enforce one of 'add', 'remove', 'set' or 'list'");
}

fn add_link_to_entry(store: &Store, matches: &ArgMatches, entry: &mut FileLockEntry) {
    Url::parse(matches.value_of("add").unwrap())
        .map_err_trace_exit(1)
        .map(|link| entry.add_external_link(store, link).map_err_trace().map_info_str("Ok"))
        .ok();
}

fn remove_link_from_entry(store: &Store, matches: &ArgMatches, entry: &mut FileLockEntry) {
    Url::parse(matches.value_of("remove").unwrap())
        .map_err_trace_exit(1)
        .map(|link| entry.remove_external_link(store, link).map_err_trace().map_info_str("Ok"))
        .ok();
}

fn set_links_for_entry(store: &Store, matches: &ArgMatches, entry: &mut FileLockEntry) {
    let links = matches
        .value_of("links")
        .map(String::from)
        .unwrap()
        .split(',')
        .map(|uri| {
            match Url::parse(uri) {
                Err(e) => {
                    warn!("Could not parse '{}' as URL, ignoring", uri);
                    trace_error(&e);
                    None
                },
                Ok(u) => Some(u),
            }
        })
        .filter_map(|x| x)
        .collect();

    entry.set_external_links(store, links)
        .map_err_trace()
        .map_info_str("Ok")
        .ok();
}

fn list_links_for_entry(store: &Store, entry: &mut FileLockEntry) {
    entry.get_external_links(store)
        .and_then(|links| {
            for (i, link) in links.enumerate() {
                match link {
                    Ok(link) => println!("{: <3}: {}", i, link),
                    Err(e)   => trace_error(&e),
                }
            }
            Ok(())
        })
        .map_err_trace()
        .map_info_str("Ok")
        .ok();
}

#[cfg(test)]
mod tests {
    use handle_internal_linking;

    use std::path::PathBuf;
    use std::ffi::OsStr;

    use toml::value::Value;
    use toml_query::read::TomlValueReadExt;
    use toml_query::error::Result as TomlQueryResult;

    use libimagrt::runtime::Runtime;
    use libimagstore::storeid::StoreId;
    use libimagstore::store::{Result as StoreResult, FileLockEntry, Entry};

    make_mock_app! {
        app "imag-link";
        modulename mock;
        version "0.5.0";
        with help "imag-link mocking app";
    }
    use self::mock::generate_test_runtime;
    use self::mock::reset_test_runtime;

    fn create_test_default_entry<'a, S: AsRef<OsStr>>(rt: &'a Runtime, name: S) -> StoreResult<StoreId> {
        let mut path = PathBuf::new();
        path.set_file_name(name);

        let default_entry = Entry::new(StoreId::new_baseless(PathBuf::from("")).unwrap()).to_str();

        let id = StoreId::new_baseless(path)?;
        let mut entry = rt.store().create(id.clone())?;
        entry.get_content_mut().push_str(&default_entry);

        Ok(id)
    }

    fn get_entry_links<'a>(entry: &'a FileLockEntry<'a>) -> TomlQueryResult<&'a Value> {
        match entry.get_header().read(&"links.internal".to_owned()) {
            Err(e) => Err(e),
            Ok(Some(v)) => Ok(v),
            Ok(None) => panic!("Didn't find 'links' in {:?}", entry),
        }
    }

    fn links_toml_value<'a, I: IntoIterator<Item = &'static str>>(links: I) -> Value {
        Value::Array(links
                         .into_iter()
                         .map(|s| Value::String(s.to_owned()))
                         .collect())
    }

    #[test]
    fn test_link_modificates() {
        let rt = generate_test_runtime(vec!["internal", "add", "test1", "test2"])
            .unwrap();

        let test_id1 = create_test_default_entry(&rt, "test1").unwrap();
        let test_id2 = create_test_default_entry(&rt, "test2").unwrap();

        handle_internal_linking(&rt);

        let test_entry1 = rt.store().get(test_id1).unwrap().unwrap();
        let test_links1 = get_entry_links(&test_entry1).unwrap();

        let test_entry2 = rt.store().get(test_id2).unwrap().unwrap();
        let test_links2 = get_entry_links(&test_entry2).unwrap();

        assert_ne!(*test_links1, links_toml_value(vec![]));
        assert_ne!(*test_links2, links_toml_value(vec![]));
    }

    #[test]
    fn test_linking_links() {
        let rt = generate_test_runtime(vec!["internal", "add", "test1", "test2"])
            .unwrap();

        let test_id1 = create_test_default_entry(&rt, "test1").unwrap();
        let test_id2 = create_test_default_entry(&rt, "test2").unwrap();

        handle_internal_linking(&rt);

        let test_entry1 = rt.store().get(test_id1).unwrap().unwrap();
        let test_links1 = get_entry_links(&test_entry1).unwrap();

        let test_entry2 = rt.store().get(test_id2).unwrap().unwrap();
        let test_links2 = get_entry_links(&test_entry2).unwrap();

        assert_eq!(*test_links1, links_toml_value(vec!["test2"]));
        assert_eq!(*test_links2, links_toml_value(vec!["test1"]));
    }

    #[test]
    fn test_multilinking() {
        let rt = generate_test_runtime(vec!["internal", "add", "test1", "test2"])
            .unwrap();

        let test_id1 = create_test_default_entry(&rt, "test1").unwrap();
        let test_id2 = create_test_default_entry(&rt, "test2").unwrap();

        handle_internal_linking(&rt);
        handle_internal_linking(&rt);

        let test_entry1 = rt.store().get(test_id1).unwrap().unwrap();
        let test_links1 = get_entry_links(&test_entry1).unwrap();

        let test_entry2 = rt.store().get(test_id2).unwrap().unwrap();
        let test_links2 = get_entry_links(&test_entry2).unwrap();

        assert_eq!(*test_links1, links_toml_value(vec!["test2"]));
        assert_eq!(*test_links2, links_toml_value(vec!["test1"]));
    }

    #[test]
    fn test_linking_more_than_two() {
        let rt = generate_test_runtime(vec!["internal", "add", "test1", "test2", "test3"])
            .unwrap();

        let test_id1 = create_test_default_entry(&rt, "test1").unwrap();
        let test_id2 = create_test_default_entry(&rt, "test2").unwrap();
        let test_id3 = create_test_default_entry(&rt, "test3").unwrap();

        handle_internal_linking(&rt);
        handle_internal_linking(&rt);

        let test_entry1 = rt.store().get(test_id1).unwrap().unwrap();
        let test_links1 = get_entry_links(&test_entry1).unwrap();

        let test_entry2 = rt.store().get(test_id2).unwrap().unwrap();
        let test_links2 = get_entry_links(&test_entry2).unwrap();

        let test_entry3 = rt.store().get(test_id3).unwrap().unwrap();
        let test_links3 = get_entry_links(&test_entry3).unwrap();

        assert_eq!(*test_links1, links_toml_value(vec!["test2", "test3"]));
        assert_eq!(*test_links2, links_toml_value(vec!["test1"]));
        assert_eq!(*test_links3, links_toml_value(vec!["test1"]));
    }

    // Remove tests

    #[test]
    fn test_linking_links_unlinking_removes_links() {
        let rt = generate_test_runtime(vec!["internal", "add", "test1", "test2"])
            .unwrap();

        let test_id1 = create_test_default_entry(&rt, "test1").unwrap();
        let test_id2 = create_test_default_entry(&rt, "test2").unwrap();

        handle_internal_linking(&rt);

        let rt = reset_test_runtime(vec!["internal", "remove", "test1", "test2"], rt)
            .unwrap();

        handle_internal_linking(&rt);

        let test_entry1 = rt.store().get(test_id1).unwrap().unwrap();
        let test_links1 = get_entry_links(&test_entry1).unwrap();

        let test_entry2 = rt.store().get(test_id2).unwrap().unwrap();
        let test_links2 = get_entry_links(&test_entry2).unwrap();

        assert_eq!(*test_links1, links_toml_value(vec![]));
        assert_eq!(*test_links2, links_toml_value(vec![]));
    }

    #[test]
    fn test_linking_and_unlinking_more_than_two() {
        let rt = generate_test_runtime(vec!["internal", "add", "test1", "test2", "test3"])
            .unwrap();

        let test_id1 = create_test_default_entry(&rt, "test1").unwrap();
        let test_id2 = create_test_default_entry(&rt, "test2").unwrap();
        let test_id3 = create_test_default_entry(&rt, "test3").unwrap();

        handle_internal_linking(&rt);

        let rt = reset_test_runtime(vec!["internal", "remove", "test1", "test2", "test3"], rt)
            .unwrap();

        handle_internal_linking(&rt);

        let test_entry1 = rt.store().get(test_id1).unwrap().unwrap();
        let test_links1 = get_entry_links(&test_entry1).unwrap();

        let test_entry2 = rt.store().get(test_id2).unwrap().unwrap();
        let test_links2 = get_entry_links(&test_entry2).unwrap();

        let test_entry3 = rt.store().get(test_id3).unwrap().unwrap();
        let test_links3 = get_entry_links(&test_entry3).unwrap();

        assert_eq!(*test_links1, links_toml_value(vec![]));
        assert_eq!(*test_links2, links_toml_value(vec![]));
        assert_eq!(*test_links3, links_toml_value(vec![]));
    }
}
