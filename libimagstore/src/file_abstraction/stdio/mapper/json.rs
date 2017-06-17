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

use std::collections::HashMap;
use std::io::Cursor;
use std::io::{Read, Write};
use std::path::PathBuf;

use serde_json;
use toml;

use error::StoreErrorKind as SEK;
use error::MapErrInto;
use super::Mapper;
use store::Result;

use libimagerror::into::IntoError;

#[derive(Deserialize, Serialize)]
struct Entry {
    header: serde_json::Value,
    content: String,
}

impl Entry {

    fn to_string(self) -> Result<String> {
        toml::to_string(&self.header)
            .map_err_into(SEK::IoError)
            .map(|hdr| {
                format!("---\n{header}---\n{content}",
                        header  = hdr,
                        content = self.content)
            })
    }

}

#[derive(Deserialize, Serialize)]
struct Document {
    version: String,
    store: HashMap<PathBuf, Entry>,
}

pub struct JsonMapper;

impl JsonMapper {

    pub fn new() -> JsonMapper {
        JsonMapper
    }

}

impl Mapper for JsonMapper {
    fn read_to_fs<R: Read>(&self, r: &mut R, hm: &mut HashMap<PathBuf, Cursor<Vec<u8>>>)   -> Result<()> {
        let mut document = {
            let mut s = String::new();
            try!(r.read_to_string(&mut s).map_err_into(SEK::IoError));
            let doc : Document = try!(serde_json::from_str(&s).map_err_into(SEK::IoError));
            doc
        };

        let _ = try!(::semver::Version::parse(&document.version)
            .map_err_into(SEK::VersionError)
            .and_then(|doc_vers| {
                // safe because cargo does not compile if crate version is not valid
                let crate_version = ::semver::Version::parse(version!()).unwrap();

                if doc_vers > crate_version {
                    Err(SEK::VersionError.into_error())
                } else {
                    Ok(())
                }
            }));

        for (key, val) in document.store.drain() {
            let res = val
                .to_string()
                .map(|vals| hm.insert(key, Cursor::new(vals.into_bytes())))
                .map(|_| ());

            let _ = try!(res);
        }

        Ok(())
    }

    fn fs_to_write<W: Write>(&self, hm: &mut HashMap<PathBuf, Cursor<Vec<u8>>>, out: &mut W) -> Result<()> {
        use util::entry_buffer_to_header_content;

        #[derive(Serialize, Deserialize)]
        struct Entry {
            header: ::toml::Value,
            content: String,
        }

        #[derive(Serialize)]
        struct OutDocument {
            version: String,
            store: HashMap<PathBuf, Entry>,
        }

        let mut doc = OutDocument {
            version: String::from(version!()),
            store:   HashMap::new(),
        };

        for (key, value) in hm.drain() {
            let res = String::from_utf8(value.into_inner())
                .map_err_into(SEK::IoError)
                .and_then(|buf| entry_buffer_to_header_content(&buf))
                .map(|(header, content)| {
                    let entry = Entry {
                        header: header,
                        content: content
                    };

                    doc.store.insert(key, entry);
                })
                .map(|_| ());

            let _ = try!(res);
        }

        serde_json::to_string(&doc)
            .map_err_into(SEK::IoError)
            .and_then(|json| out.write(&json.into_bytes()).map_err_into(SEK::IoError))
            .and_then(|_| out.flush().map_err_into(SEK::IoError))
            .map(|_| ())
    }
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_json_to_fs() {
        let json = r#"
        { "version": "0.3.0",
          "store": {
            "/example": {
                "header": {
                    "imag": {
                        "version": "0.3.0"
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
version = "0.3.0"
---
hi there!"#;
            hm.insert(PathBuf::from("/example"), Cursor::new(String::from(content).into_bytes()));
            hm
        };

        let io_res = mapper.fs_to_write(&mut hm, &mut out);
        assert!(io_res.is_ok());

        let example = r#"
        {
            "version": "0.3.0",
            "store": {
                "/example": {
                    "header": {
                        "imag": {
                            "version": "0.3.0"
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

