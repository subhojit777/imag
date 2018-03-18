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
#[cfg(test)] extern crate toml;
#[cfg(test)] extern crate toml_query;
#[cfg(test)] extern crate env_logger;

extern crate libimagentrylink;
#[macro_use] extern crate libimagrt;
extern crate libimagstore;
extern crate libimagerror;

#[cfg(test)]
#[macro_use]
extern crate libimagutil;

#[cfg(not(test))]
extern crate libimagutil;

use std::io::Write;
use std::path::PathBuf;
use std::process::exit;

use libimagentrylink::external::ExternalLinker;
use libimagentrylink::internal::InternalLinker;
use libimagentrylink::internal::store_check::StoreLinkConsistentExt;
use libimagentrylink::error::LinkError as LE;
use libimagerror::trace::{MapErrTrace, trace_error};
use libimagerror::exit::ExitUnwrap;
use libimagerror::io::ToExitCode;
use libimagrt::runtime::Runtime;
use libimagrt::setup::generate_runtime_setup;
use libimagstore::error::StoreError;
use libimagstore::store::FileLockEntry;
use libimagstore::storeid::StoreId;
use libimagutil::warn_exit::warn_exit;
use libimagutil::warn_result::*;

use url::Url;

mod ui;

use ui::build_ui;

fn main() {
    let version = make_imag_version!();
    let rt = generate_runtime_setup("imag-link",
                                    &version,
                                    "Link entries",
                                    build_ui);
    if rt.cli().is_present("check-consistency") {
        let exit_code = match rt.store().check_link_consistency() {
            Ok(_) => {
                info!("Store is consistent");
                0
            }
            Err(e) => {
                trace_error(&e);
                1
            }
        };
        ::std::process::exit(exit_code);
    }

    let _ = rt.cli()
        .subcommand_name()
        .map(|name| {
            match name {
                "remove" => remove_linking(&rt),
                "unlink" => unlink(&rt),
                "list"   => list_linkings(&rt),
                _ => panic!("BUG"),
            }
        })
        .or_else(|| {
            if let (Some(from), Some(to)) = (rt.cli().value_of("from"), rt.cli().values_of("to")) {
                Some(link_from_to(&rt, from, to))
            } else {
                warn_exit("No commandline call", 1)
            }
        })
        .ok_or(LE::from("No commandline call".to_owned()))
        .map_err_trace_exit_unwrap(1);
}

fn get_entry_by_name<'a>(rt: &'a Runtime, name: &str) -> Result<Option<FileLockEntry<'a>>, StoreError> {
    use libimagstore::storeid::StoreId;

    debug!("Getting: {:?}", name);
    let result = StoreId::new(Some(rt.store().path().clone()), PathBuf::from(name))
        .and_then(|id| rt.store().get(id));

    debug!(" => : {:?}", result);
    result
}

fn link_from_to<'a, I>(rt: &'a Runtime, from: &'a str, to: I)
    where I: Iterator<Item = &'a str>
{
    let mut from_entry = match get_entry_by_name(rt, from).map_err_trace_exit_unwrap(1) {
        Some(e) => e,
        None    => {
            debug!("No 'from' entry");
            warn_exit("No 'from' entry", 1)
        },
    };

    for entry in to {
        debug!("Handling 'to' entry: {:?}", entry);
        if PathBuf::from(entry).exists() {
            debug!("Linking externally: {:?} -> {:?}", from, entry);
            let url = Url::parse(entry).unwrap_or_else(|e| {
                error!("Error parsing URL: {:?}", e);
                ::std::process::exit(1);
            });

            let _ = from_entry
                .add_external_link(rt.store(), url)
                .map_err_trace_exit_unwrap(1);
        } else {
            debug!("Linking internally: {:?} -> {:?}", from, entry);

            let from_id = StoreId::new_baseless(PathBuf::from(from)).map_err_trace_exit_unwrap(1);
            let entr_id = StoreId::new_baseless(PathBuf::from(entry)).map_err_trace_exit_unwrap(1);

            if from_id == entr_id {
                error!("Cannot link entry with itself. Exiting");
                ::std::process::exit(1)
            }

            let mut to_entry = match rt.store().get(entr_id).map_err_trace_exit_unwrap(1) {
                Some(e) => e,
                None    => {
                    warn!("No 'to' entry: {}", entry);
                    ::std::process::exit(1)
                },
            };
            let _ = from_entry
                .add_internal_link(&mut to_entry)
                .map_err_trace_exit_unwrap(1);
        }

        info!("Ok: {} -> {}", from, entry);
    }
}

fn remove_linking(rt: &Runtime) {

    fn get_from_entry<'a>(rt: &'a Runtime) -> Option<FileLockEntry<'a>> {
        rt.cli()
            .subcommand_matches("remove")
            .unwrap() // safe, we know there is an "remove" subcommand
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

    let mut from = match get_from_entry(&rt) {
        None => warn_exit("No 'from' entry", 1),
        Some(s) => s,
    };

    rt.cli()
        .subcommand_matches("remove")
        .unwrap()
        .values_of("to")
        .map(|values| {
            for (entry, value) in values.map(|v| (get_entry_by_name(rt, v), v)) {
                match entry {
                    Err(e) => trace_error(&e),
                    Ok(Some(mut to_entry)) => {
                        let _ = to_entry
                            .remove_internal_link(&mut from)
                            .map_err_trace_exit_unwrap(1);
                    },
                    Ok(None) => {
                        // looks like this is not an entry, but a filesystem URI and therefor an
                        // external link...?
                        if PathBuf::from(value).is_file() {
                            let url = Url::parse(value).unwrap_or_else(|e| {
                                error!("Error parsing URL: {:?}", e);
                                ::std::process::exit(1);
                            });
                            from.remove_external_link(rt.store(), url).map_err_trace_exit_unwrap(1);
                            info!("Ok: {}", value);
                        } else {
                            warn!("Entry not found: {:?}", value);
                        }
                    }
                }
            }
        });
}

fn unlink(rt: &Runtime) {
    use libimagerror::iter::TraceIterator;
    use libimagstore::iter::get::StoreIdGetIteratorExtension;

    let _ = rt
        .cli()
        .subcommand_matches("unlink")
        .unwrap() // checked in main()
        .values_of("from")
        .unwrap() // checked by clap
        .map(PathBuf::from)
        .collect::<Vec<PathBuf>>().into_iter() // for lifetime inference
        .map(StoreId::new_baseless)
        .unwrap_with(|e| { trace_error(&e); exit(1) })
        .into_get_iter(rt.store())
        .unwrap_with(|e| { trace_error(&e); exit(1) })
        .filter_map(|e| e)
        .map(|mut entry| entry.unlink(rt.store()))
        .unwrap_with(|e| { trace_error(&e); exit(1) })
        .collect::<Vec<_>>();
}

fn list_linkings(rt: &Runtime) {
    let cmd = rt.cli()
        .subcommand_matches("list")
        .unwrap(); // safed by clap

    let list_externals  = cmd.is_present("list-externals-too");

    for entry in cmd.values_of("entries").unwrap() { // safed by clap
        match rt.store().get(PathBuf::from(entry)) {
            Ok(Some(entry)) => {
                let mut i = 0;

                for link in entry.get_internal_links().map_err_trace_exit_unwrap(1) {
                    let link = link
                        .to_str()
                        .map_warn_err(|e| format!("Failed to convert StoreId to string: {:?}", e))
                        .ok();

                    if let Some(link) = link {
                        let _ = writeln!(rt.stdout(), "{: <3}: {}", i, link)
                            .to_exit_code()
                            .unwrap_or_exit();
                        i += 1;
                    }
                }

                if list_externals {
                    entry.get_external_links(rt.store())
                        .map_err_trace_exit_unwrap(1)
                        .enumerate()
                        .for_each(|(i, link)| {
                            let link = link
                                .map_err_trace_exit_unwrap(1)
                                .into_string();

                            let _ = writeln!(rt.stdout(), "{: <3}: {}", i, link)
                                .to_exit_code()
                                .unwrap_or_exit();

                        })
                }
            },
            Ok(None)        => warn!("Not found: {}", entry),
            Err(e)          => trace_error(&e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::link_from_to;
    use super::remove_linking;

    use std::path::PathBuf;
    use std::ffi::OsStr;

    use toml::value::Value;
    use toml_query::read::TomlValueReadExt;
    use toml_query::error::Result as TomlQueryResult;

    use libimagrt::runtime::Runtime;
    use libimagstore::storeid::StoreId;
    use libimagstore::store::{Result as StoreResult, FileLockEntry, Entry};

    fn setup_logging() {
        let _ = ::env_logger::try_init();
    }

    make_mock_app! {
        app "imag-link";
        modulename mock;
        version env!("CARGO_PKG_VERSION");
        with help "imag-link mocking app";
    }
    use self::mock::generate_test_runtime;
    use self::mock::reset_test_runtime;

    fn create_test_default_entry<'a, S: AsRef<OsStr>>(rt: &'a Runtime, name: S) -> StoreResult<StoreId> {
        let mut path = PathBuf::new();
        path.set_file_name(name);

        let default_entry = Entry::new(StoreId::new_baseless(PathBuf::from("")).unwrap()).to_str();

        debug!("Default entry constructed");

        let id = StoreId::new_baseless(path)?;
        debug!("StoreId constructed: {:?}", id);

        let mut entry = rt.store().create(id.clone())?;

        debug!("Entry constructed: {:?}", id);
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
        setup_logging();
        let rt = generate_test_runtime(vec!["internal", "test1", "test2"])
            .unwrap();

        debug!("Runtime created");

        let test_id1 = create_test_default_entry(&rt, "test1").unwrap();
        let test_id2 = create_test_default_entry(&rt, "test2").unwrap();

        debug!("Entries created");

        link_from_to(&rt, "test1", vec!["test2"].into_iter());

        debug!("Linking done");

        let test_entry1 = rt.store().get(test_id1).unwrap().unwrap();
        let test_links1 = get_entry_links(&test_entry1).unwrap();

        let test_entry2 = rt.store().get(test_id2).unwrap().unwrap();
        let test_links2 = get_entry_links(&test_entry2).unwrap();

        debug!("Asserting");

        assert_ne!(*test_links1, links_toml_value(vec![]));
        assert_ne!(*test_links2, links_toml_value(vec![]));
    }

    #[test]
    fn test_linking_links() {
        setup_logging();
        let rt = generate_test_runtime(vec!["internal", "test1", "test2"])
            .unwrap();

        debug!("Runtime created");

        let test_id1 = create_test_default_entry(&rt, "test1").unwrap();
        let test_id2 = create_test_default_entry(&rt, "test2").unwrap();

        debug!("Test entries created");

        link_from_to(&rt, "test1", vec!["test2"].into_iter());

        debug!("Linking done");

        let test_entry1 = rt.store().get(test_id1).unwrap().unwrap();
        let test_links1 = get_entry_links(&test_entry1).unwrap();

        let test_entry2 = rt.store().get(test_id2).unwrap().unwrap();
        let test_links2 = get_entry_links(&test_entry2).unwrap();

        debug!("Asserting");

        assert_eq!(*test_links1, links_toml_value(vec!["test2"]));
        assert_eq!(*test_links2, links_toml_value(vec!["test1"]));
    }

    #[test]
    fn test_multilinking() {
        setup_logging();
        let rt = generate_test_runtime(vec!["internal", "test1", "test2"])
            .unwrap();

        debug!("Runtime created");

        let test_id1 = create_test_default_entry(&rt, "test1").unwrap();
        let test_id2 = create_test_default_entry(&rt, "test2").unwrap();

        debug!("Test entries created");

        link_from_to(&rt, "test1", vec!["test2"].into_iter());
        link_from_to(&rt, "test1", vec!["test2"].into_iter());

        debug!("Linking done");

        let test_entry1 = rt.store().get(test_id1).unwrap().unwrap();
        let test_links1 = get_entry_links(&test_entry1).unwrap();

        let test_entry2 = rt.store().get(test_id2).unwrap().unwrap();
        let test_links2 = get_entry_links(&test_entry2).unwrap();

        debug!("Asserting");

        assert_eq!(*test_links1, links_toml_value(vec!["test2"]));
        assert_eq!(*test_links2, links_toml_value(vec!["test1"]));
    }

    #[test]
    fn test_linking_more_than_two() {
        setup_logging();
        let rt = generate_test_runtime(vec!["internal", "test1", "test2", "test3"])
            .unwrap();

        debug!("Runtime created");

        let test_id1 = create_test_default_entry(&rt, "test1").unwrap();
        let test_id2 = create_test_default_entry(&rt, "test2").unwrap();
        let test_id3 = create_test_default_entry(&rt, "test3").unwrap();

        debug!("Test entries created");

        link_from_to(&rt, "test1", vec!["test2", "test3"].into_iter());
        link_from_to(&rt, "test1", vec!["test2", "test3"].into_iter());

        debug!("Linking done");

        let test_entry1 = rt.store().get(test_id1).unwrap().unwrap();
        let test_links1 = get_entry_links(&test_entry1).unwrap();

        let test_entry2 = rt.store().get(test_id2).unwrap().unwrap();
        let test_links2 = get_entry_links(&test_entry2).unwrap();

        let test_entry3 = rt.store().get(test_id3).unwrap().unwrap();
        let test_links3 = get_entry_links(&test_entry3).unwrap();

        debug!("Asserting");

        assert_eq!(*test_links1, links_toml_value(vec!["test2", "test3"]));
        assert_eq!(*test_links2, links_toml_value(vec!["test1"]));
        assert_eq!(*test_links3, links_toml_value(vec!["test1"]));
    }

    // Remove tests

    #[test]
    fn test_linking_links_unlinking_removes_links() {
        setup_logging();
        let rt = generate_test_runtime(vec!["internal", "test1", "test2"])
            .unwrap();

        debug!("Runtime created");

        let test_id1 = create_test_default_entry(&rt, "test1").unwrap();
        let test_id2 = create_test_default_entry(&rt, "test2").unwrap();

        debug!("Test entries created");

        link_from_to(&rt, "test1", vec!["test2"].into_iter());

        debug!("Linking done");

        let rt = reset_test_runtime(vec!["remove", "test1", "test2"], rt)
            .unwrap();

        remove_linking(&rt);

        debug!("Linking removed");

        let test_entry1 = rt.store().get(test_id1).unwrap().unwrap();
        let test_links1 = get_entry_links(&test_entry1).unwrap();

        let test_entry2 = rt.store().get(test_id2).unwrap().unwrap();
        let test_links2 = get_entry_links(&test_entry2).unwrap();

        debug!("Asserting");

        assert_eq!(*test_links1, links_toml_value(vec![]));
        assert_eq!(*test_links2, links_toml_value(vec![]));
    }

    #[test]
    fn test_linking_and_unlinking_more_than_two() {
        setup_logging();
        let rt = generate_test_runtime(vec!["internal", "test1", "test2", "test3"])
            .unwrap();

        debug!("Runtime created");

        let test_id1 = create_test_default_entry(&rt, "test1").unwrap();
        let test_id2 = create_test_default_entry(&rt, "test2").unwrap();
        let test_id3 = create_test_default_entry(&rt, "test3").unwrap();

        debug!("Test entries created");

        link_from_to(&rt, "test1", vec!["test2", "test3"].into_iter());

        debug!("linking done");

        let rt = reset_test_runtime(vec!["remove", "test1", "test2", "test3"], rt)
            .unwrap();

        remove_linking(&rt);

        debug!("linking removed");

        let test_entry1 = rt.store().get(test_id1).unwrap().unwrap();
        let test_links1 = get_entry_links(&test_entry1).unwrap();

        let test_entry2 = rt.store().get(test_id2).unwrap().unwrap();
        let test_links2 = get_entry_links(&test_entry2).unwrap();

        let test_entry3 = rt.store().get(test_id3).unwrap().unwrap();
        let test_links3 = get_entry_links(&test_entry3).unwrap();

        debug!("Asserting");

        assert_eq!(*test_links1, links_toml_value(vec![]));
        assert_eq!(*test_links2, links_toml_value(vec![]));
        assert_eq!(*test_links3, links_toml_value(vec![]));
    }
}
