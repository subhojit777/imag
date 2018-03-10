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

extern crate libimagentrylink;
extern crate libimagrt;
extern crate libimagstore;
extern crate libimagerror;

#[cfg(test)]
#[macro_use]
extern crate libimagutil;

#[cfg(not(test))]
extern crate libimagutil;

use std::path::PathBuf;

use libimagentrylink::external::ExternalLinker;
use libimagentrylink::internal::InternalLinker;
use libimagentrylink::internal::store_check::StoreLinkConsistentExt;
use libimagentrylink::error::LinkError as LE;
use libimagerror::trace::{MapErrTrace, trace_error, trace_error_exit};
use libimagrt::runtime::Runtime;
use libimagrt::setup::generate_runtime_setup;
use libimagstore::error::StoreError;
use libimagstore::store::FileLockEntry;
use libimagutil::warn_exit::warn_exit;
use libimagutil::warn_result::*;

use url::Url;

mod ui;

use ui::build_ui;

fn main() {
    let rt = generate_runtime_setup("imag-link",
                                    env!("CARGO_PKG_VERSION"),
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

    StoreId::new(Some(rt.store().path().clone()), PathBuf::from(name))
        .and_then(|id| rt.store().get(id))
}

fn link_from_to<'a, I>(rt: &'a Runtime, from: &'a str, to: I)
    where I: Iterator<Item = &'a str>
{
    let mut from_entry = match get_entry_by_name(rt, from) {
        Ok(Some(e)) => e,
        Ok(None)    => warn_exit("No 'from' entry", 1),
        Err(e)      => trace_error_exit(&e, 1),
    };

    for entry in to {
        if PathBuf::from(entry).exists() {
            debug!("Linking externally: {:?} -> {:?}", from, entry);
            let url = Url::parse(entry).map_err_trace_exit_unwrap(1);
            let _ = from_entry
                .add_external_link(rt.store(), url)
                .map_err_trace_exit_unwrap(1);
        } else {
            debug!("Linking internally: {:?} -> {:?}", from, entry);
            let mut to_entry = match get_entry_by_name(rt, entry) {
                Ok(Some(e)) => e,
                Ok(None)    => {
                    warn!("No 'to' entry: {}", entry);
                    ::std::process::exit(1)
                },
                Err(e)      => trace_error_exit(&e, 1),
            };
            let _ = from_entry
                .add_internal_link(&mut to_entry)
                .map_err_trace_exit_unwrap(1);
        }

        info!("Ok: {} -> {}", from, entry);
    }

    info!("Ok");
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
                        if let Err(e) = to_entry.remove_internal_link(&mut from) {
                            trace_error_exit(&e, 1);
                        }
                    },
                    Ok(None) => {
                        // looks like this is not an entry, but a filesystem URI and therefor an
                        // external link...?
                        if PathBuf::from(value).is_file() {
                            let url = Url::parse(value).map_err_trace_exit_unwrap(1);
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

fn list_linkings(rt: &Runtime) {
    let cmd = rt.cli()
        .subcommand_matches("list")
        .unwrap(); // safed by clap

    let list_externals = cmd.is_present("list-externals-too");

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
                        println!("{: <3}: {}", i, link);
                        i += 1;
                    }
                }

                if list_externals {
                    for link in entry.get_external_links(rt.store()).map_err_trace_exit_unwrap(1) {
                        let link = link
                            .map_err_trace_exit_unwrap(1)
                            .into_string();

                        println!("{: <3}: {}", i, link);
                        i += 1;
                    }
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

    make_mock_app! {
        app "imag-link";
        modulename mock;
        version "0.6.3";
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

        link_from_to(&rt, "test1", vec!["test2"].into_iter());

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

        link_from_to(&rt, "test1", vec!["test2"].into_iter());

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

        link_from_to(&rt, "test1", vec!["test2"].into_iter());
        link_from_to(&rt, "test1", vec!["test2"].into_iter());

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

        link_from_to(&rt, "test1", vec!["test2", "test3"].into_iter());
        link_from_to(&rt, "test1", vec!["test2", "test3"].into_iter());

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

        link_from_to(&rt, "test1", vec!["test2"].into_iter());

        let rt = reset_test_runtime(vec!["remove", "test1", "test2"], rt)
            .unwrap();

        remove_linking(&rt);

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

        link_from_to(&rt, "test1", vec!["test2", "test3"].into_iter());

        let rt = reset_test_runtime(vec!["remove", "test1", "test2", "test3"], rt)
            .unwrap();

        remove_linking(&rt);

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
