use std::collections::BTreeMap;
use std::path::PathBuf;
use std::str::Split;

use clap::ArgMatches;
use toml::Value;

use libimagstore::store::EntryHeader;
use libimagrt::runtime::Runtime;
use libimagutil::key_value_split::IntoKeyValue;

pub fn build_entry_path(rt: &Runtime, path_elem: &str) -> PathBuf {
    debug!("Building path...");
    let mut path = rt.store().path().clone();

    if path_elem.chars().next() == Some('/') {
        path.push(&path_elem[1..path_elem.len()]);
    } else {
        path.push(path_elem);
    }

    path
}

pub fn build_toml_header(matches: &ArgMatches, mut header: EntryHeader) -> EntryHeader {
    debug!("Building header from cli spec");
    if let Some(headerspecs) = matches.values_of("header") {
        let mut main = header.toml_mut();
        debug!("headerspec = {:?}", headerspecs);
        let kvs = headerspecs.into_iter()
                            .filter_map(|hs| {
                                debug!("- Processing: '{}'", hs);
                                let kv = String::from(hs).into_kv();
                                debug!("-        got: '{:?}'", kv);
                                kv
                            });
        for tpl in kvs {
            let (key, value) = tpl.into();
            debug!("Splitting: {:?}", key);
            let mut split = key.split(".");
            let current = split.next();
            if current.is_some() {
                insert_key_into(String::from(current.unwrap()), &mut split, value, &mut main);
            }
        }
    }
    debug!("Header = {:?}", header);
    header
}

fn insert_key_into(current: String,
                   rest_path: &mut Split<&str>,
                   value: String,
                   map: &mut BTreeMap<String, Value>) {
    let next = rest_path.next();

    if next.is_none() {
        debug!("Inserting into {:?} = {:?}", current, value);
        map.insert(current, parse_value(value));
    } else {
        debug!("Inserting into {:?} ... = {:?}", current, value);
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
            debug!("Inserting submap = {:?}", submap);
            map.insert(current, Value::Table(submap));
        }
    }
}

fn parse_value(value: String) -> Value {
    fn is_ary(v: &String) -> bool {
        v.chars().next() == Some('[') && v.chars().last() == Some(']') && v.len() >= 3
    }

    if value == "true" {
        debug!("Building Boolean out of: {:?}...", value);
        Value::Boolean(true)
    } else if value == "false" {
        debug!("Building Boolean out of: {:?}...", value);
        Value::Boolean(false)
    } else if is_ary(&value) {
        debug!("Building Array out of: {:?}...", value);
        let sub = &value[1..(value.len()-1)];
        Value::Array(sub.split(",").map(|v| parse_value(String::from(v))).collect())
    } else {
        debug!("Building String out of: {:?}...", value);
        Value::String(value)
    }
}

