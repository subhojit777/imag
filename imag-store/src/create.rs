use std::collections::BTreeMap;
use std::path::PathBuf;
use std::io::stdin;
use std::fs::OpenOptions;
use std::result::Result as RResult;
use std::io::Read;
use std::ops::DerefMut;
use std::str::Split;

use clap::ArgMatches;
use toml::Table;
use toml::Value;

use libimagrt::runtime::Runtime;
use libimagstore::store::Entry;
use libimagstore::store::EntryHeader;
use libimagutil::key_value_split::IntoKeyValue;

use error::StoreError;
use error::StoreErrorKind;
use util::build_entry_path;

type Result<T> = RResult<T, StoreError>;

pub fn create(rt: &Runtime) {
    rt.cli()
        .subcommand_matches("create")
        .map(|scmd| {
            debug!("Found 'create' subcommand...");

            // unwrap is safe as value is required
            let path = build_entry_path(rt, scmd.value_of("path").unwrap());
            debug!("path = {:?}", path);

            scmd.subcommand_matches("entry")
                .map(|entry| create_from_cli_spec(rt, scmd, &path))
                .ok_or(()) // hackythehackhack
                .map_err(|_| {
                    create_from_source(rt, scmd, &path)
                        .unwrap_or_else(|e| debug!("Error building Entry: {:?}", e))
                });
        });
}

fn create_from_cli_spec(rt: &Runtime, matches: &ArgMatches, path: &PathBuf) -> Result<()> {
    let content = matches.subcommand_matches("entry")
        .map(|entry_subcommand| {
            debug!("Found entry subcommand, parsing content");
            entry_subcommand
                .value_of("content")
                .map(String::from)
                .unwrap_or_else(|| {
                    entry_subcommand
                        .value_of("content-from")
                        .map(|src| entry_from_raw(src))
                        .unwrap_or(String::new())
                })
        })
        .unwrap_or_else(|| {
            debug!("Didn't find entry subcommand, getting raw content");
            matches.value_of("from-raw")
                .map(|raw_src| entry_from_raw(raw_src))
                .unwrap_or(String::new())
        });

    debug!("Got content with len = {}", content.len());

    rt.store()
        .create(PathBuf::from(path))
        .map(|mut element| {
            {
                let mut e_content = element.get_content_mut();
                *e_content = content;
                debug!("New content set");
            }
            {
                let mut e_header  = element.get_header_mut();
                matches.subcommand_matches("entry")
                    .map(|entry_matches| {
                        *e_header = build_toml_header(entry_matches, EntryHeader::new());
                        debug!("New header set");
                    });
            }
        })
        .map_err(|e| StoreError::new(StoreErrorKind::BackendError, Some(Box::new(e))))
}

fn create_from_source(rt: &Runtime, matches: &ArgMatches, path: &PathBuf) -> Result<()> {
    let content = matches
        .value_of("from-raw")
        .ok_or(StoreError::new(StoreErrorKind::NoCommandlineCall, None))
        .map(|raw_src| entry_from_raw(raw_src));

    if content.is_err() {
        return content.map(|_| ());
    }
    let content = content.unwrap();
    debug!("Content with len = {}", content.len());

    Entry::from_str(path.clone(), &content[..])
        .map(|mut new_e| {
            rt.store()
                .create(path.clone())
                .map(|mut old_e| {
                    *old_e.deref_mut() = new_e;
                });

            debug!("Entry build");
        })
        .map_err(|serr| StoreError::new(StoreErrorKind::BackendError, Some(Box::new(serr))))
}

fn entry_from_raw(raw_src: &str) -> String {
    let mut content = String::new();
    if raw_src == "-" {
        debug!("Reading entry from stdin");
        let res = stdin().read_to_string(&mut content);
        debug!("Read {:?} bytes", res);
    } else {
        debug!("Reading entry from file at {:?}", raw_src);
        OpenOptions::new()
            .read(true)
            .write(false)
            .create(false)
            .open(raw_src)
            .and_then(|mut f| f.read_to_string(&mut content));
    }
    content
}

fn build_toml_header(matches: &ArgMatches, header: EntryHeader) -> EntryHeader {
    if let Some(headerspecs) = matches.values_of("header") {
        let mut main = BTreeMap::new();
        for tpl in headerspecs.into_iter().filter_map(|hs| String::from(hs).into_kv()) {
            let (key, value) = tpl.into();
            let mut split = key.split(".");
            let current = split.next();
            if current.is_some() {
                insert_key_into(String::from(current.unwrap()), &mut split, value, &mut main);
            }
        }
    }
    header
}

fn insert_key_into(current: String,
                   rest_path: &mut Split<&str>,
                   value: String,
                   map: &mut BTreeMap<String, Value>) {
    let next = rest_path.next();

    if next.is_none() {
        map.insert(current, parse_value(value));
    } else {
        if map.contains_key(&current) {
            match map.get_mut(&current).unwrap() {
                &mut Value::Table(ref mut t) => {
                    insert_key_into(String::from(next.unwrap()), rest_path, value, t);
                },
                _ => unreachable!(),
            }
        } else {
            let mut submap = BTreeMap::new();
            insert_key_into(String::from(next.unwrap()), rest_path, value, &mut submap);
            map.insert(current, Value::Table(submap));
        }
    }
}

fn parse_value(value: String) -> Value {
    fn is_ary(v: &String) -> bool {
        v.chars().next() == Some('[') && v.chars().last() == Some(']') && v.len() >= 3
    }

    if value == "true" {
        Value::Boolean(true)
    } else if value == "false" {
        Value::Boolean(false)
    } else if is_ary(&value) {
        let sub = &value[1..(value.len()-1)];
        Value::Array(sub.split(",").map(|v| parse_value(String::from(v))).collect())
    } else {
        Value::String(value)
    }
}

