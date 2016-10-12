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

use std::borrow::Cow;
use std::collections::btree_map::{BTreeMap, Entry};
use std::str::Split;

use clap::ArgMatches;
use toml::Value;

use libimagstore::store::EntryHeader;
use libimagutil::key_value_split::IntoKeyValue;

pub fn build_toml_header(matches: &ArgMatches, header: EntryHeader) -> EntryHeader {
    debug!("Building header from cli spec");
    if let Some(headerspecs) = matches.values_of("header") {
        let mut main = header.into();
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
            let mut split = key.split('.');
            let current = split.next();
            if current.is_some() {
                insert_key_into(String::from(current.unwrap()), &mut split, Cow::Owned(value), &mut main);
            }
        }

        debug!("Header = {:?}", main);
        EntryHeader::from(main)
    } else {
        debug!("Header = {:?}", header);
        header
    }
}

fn insert_key_into<'a>(current: String,
                   rest_path: &mut Split<char>,
                   value: Cow<'a, str>,
                   map: &mut BTreeMap<String, Value>) {
    let next = rest_path.next();

    if next.is_none() {
        debug!("Inserting into {:?} = {:?}", current, value);
        map.insert(current, parse_value(value));
    } else {
        debug!("Inserting into {:?} ... = {:?}", current, value);
        match map.entry(current) {
            Entry::Occupied(ref mut e) => {
                match *e.get_mut() {
                    Value::Table(ref mut t) => {
                        insert_key_into(String::from(next.unwrap()), rest_path, value, t);
                    },
                    _ => unreachable!(),
                }
            },
            Entry::Vacant(v) => { v.insert(Value::Table( {
                let mut submap = BTreeMap::new();
                insert_key_into(String::from(next.unwrap()), rest_path, value, &mut submap);
                debug!("Inserting submap = {:?}", submap);
                submap }));
            }
        }
    }
}

fn parse_value(value: Cow<str>) -> Value {
    use std::str::FromStr;

    fn is_ary(v: &str) -> bool {
        v.starts_with('[') && v.ends_with(']') && v.len() >= 3
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
        Value::Array(sub.split(',').map(|x| parse_value(Cow::from(x))).collect())
    } else {
        FromStr::from_str(&value[..])
            .map(|i: i64| {
                debug!("Building Integer out of: {:?}...", value);
                Value::Integer(i)
            })
            .unwrap_or_else(|_| {
                FromStr::from_str(&value[..])
                    .map(|f: f64| {
                        debug!("Building Float out of: {:?}...", value);
                        Value::Float(f)
                    })
                    .unwrap_or_else(|_| {
                        debug!("Building String out of: {:?}...", value);
                        Value::String(value.into_owned())
                    })
            })
    }
}

