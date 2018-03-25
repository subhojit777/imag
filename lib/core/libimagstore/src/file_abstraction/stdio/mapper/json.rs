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

use std::collections::HashMap;
use std::io::{Read, Write};
use std::path::PathBuf;

use serde_json;
use toml;

use error::StoreErrorKind as SEK;
use error::StoreError as SE;
use error::ResultExt;
use super::Mapper;
use store::Result;
use store::Entry;
use storeid::StoreId;

#[derive(Debug, Deserialize, Serialize)]
struct BackendEntry {
    header: serde_json::Value,
    content: String,
}

impl BackendEntry {

    fn to_string(self) -> Result<String> {
        toml::to_string(&self.header)
            .chain_err(|| SEK::IoError)
            .map(|hdr| {
                format!("---\n{header}---\n{content}",
                        header  = hdr,
                        content = self.content)
            })
    }

}

#[derive(Debug, Deserialize, Serialize)]
struct Document {
    version: String,
    store: HashMap<PathBuf, BackendEntry>,
}

pub struct JsonMapper;

impl JsonMapper {

    pub fn new() -> JsonMapper {
        JsonMapper
    }

}

impl Mapper for JsonMapper {
    fn read_to_fs<R: Read>(&self, r: &mut R, hm: &mut HashMap<PathBuf, Entry>)   -> Result<()> {
        let mut document = {
            debug!("Reading Document");
            let mut s = String::new();
            r.read_to_string(&mut s).chain_err(|| SEK::IoError)?;
            debug!("Document = {:?}", s);
            debug!("Parsing Document");
            let doc : Document = serde_json::from_str(&s).chain_err(|| SEK::IoError)?;
            debug!("Document = {:?}", doc);
            doc
        };

        let _ = ::semver::Version::parse(&document.version)
            .chain_err(|| SEK::VersionError)
            .and_then(|doc_vers| {
                // safe because cargo does not compile if crate version is not valid
                let crate_version = ::semver::Version::parse(env!("CARGO_PKG_VERSION")).unwrap();

                debug!("Document version vs. own version: {doc_vers} > {crate_vers}",
                       doc_vers = doc_vers,
                       crate_vers = crate_version);

                if doc_vers > crate_version {
                    Err(SE::from_kind(SEK::VersionError))
                } else {
                    Ok(())
                }
            })?;

        for (key, val) in document.store.drain() {
            debug!("(key, value) ({:?}, {:?})", key, val);
            let res = val
                .to_string()
                .and_then(|vals| {
                    debug!("value string = {:?}", vals);
                    StoreId::new_baseless(key.clone())
                        .and_then(|id| Entry::from_str(id, &vals))
                        .map(|entry| hm.insert(key, entry))
                })
                .map(|_| ());

            let _ = res?;
        }

        Ok(())
    }

    fn fs_to_write<W: Write>(&self, hm: &mut HashMap<PathBuf, Entry>, out: &mut W) -> Result<()> {
        #[derive(Serialize)]
        struct BackendEntry {
            header: ::toml::Value,
            content: String,
        }

        impl BackendEntry {
            fn construct_from(e: Entry) -> BackendEntry {
                BackendEntry {
                    header:  e.get_header().clone(),
                    content: e.get_content().clone(),
                }
            }
        }

        #[derive(Serialize)]
        struct OutDocument {
            version: String,
            store: HashMap<PathBuf, BackendEntry>,
        }

        let mut store = HashMap::new();
        for (key, value) in hm.drain() {
            store.insert(key, BackendEntry::construct_from(value));
        }

        let doc = OutDocument {
            version: String::from(env!("CARGO_PKG_VERSION")),
            store:   store,
        };

        serde_json::to_string(&doc)
            .chain_err(|| SEK::IoError)
            .and_then(|json| out.write(&json.into_bytes()).chain_err(|| SEK::IoError))
            .and_then(|_| out.flush().chain_err(|| SEK::IoError))
            .map(|_| ())
    }
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_empty_json_to_fs() {
        let json = r#"{"version":"0.6.4","store":{}}"#;
        let mut json = Cursor::new(String::from(json).into_bytes());
        let mapper   = JsonMapper::new();
        let mut hm   = HashMap::new();

        let io_res  = mapper.read_to_fs(&mut json, &mut hm);
        assert!(io_res.is_ok());
        assert!(hm.is_empty());
    }

    #[test]
    fn test_json_to_fs() {
        let json = r#"
        { "version": "0.6.4",
          "store": {
            "example": {
                "header": {
                    "imag": {
                        "version": "0.6.4"
                    }
                },
                "content": "test"
            }
          }
        }
        "#;
        let mut json = Cursor::new(String::from(json).into_bytes());
        let mapper   = JsonMapper::new();
        let mut hm   = HashMap::new();

        let io_res  = mapper.read_to_fs(&mut json, &mut hm);
        assert!(io_res.is_ok());

        assert_eq!(1, hm.len()); // we should have exactly one entry
    }

    #[test]
    fn test_fs_to_json() {
        let mapper                    = JsonMapper::new();
        let mut out : Cursor<Vec<u8>> = Cursor::new(vec![]);

        let mut hm = {
            let mut hm = HashMap::new();
            let content = r#"---
[imag]
version = "0.6.4"
---
hi there!"#;

            let id = PathBuf::from("example");
            let entry = Entry::from_str(id.clone(), content).unwrap();
            hm.insert(id, entry);
            hm
        };

        let io_res = mapper.fs_to_write(&mut hm, &mut out);
        assert!(io_res.is_ok());

        let example = r#"
        {
            "version": "0.6.4",
            "store": {
                "example": {
                    "header": {
                        "imag": {
                            "version": "0.6.4"
                        }
                    },
                    "content": "hi there!"
                }
            }
        }
        "#;

        let example_json : ::serde_json::Value = ::serde_json::from_str(example).unwrap();

        let output_json = String::from_utf8(out.into_inner()).unwrap();
        let output_json : ::serde_json::Value = ::serde_json::from_str(&output_json).unwrap();

        assert_eq!(example_json, output_json);
    }
}

