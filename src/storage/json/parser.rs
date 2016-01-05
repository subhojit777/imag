use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fmt;

use serde_json::{Value, from_str};
use serde_json::error::Result as R;
use serde_json::Serializer;
use serde::ser::Serialize;
use serde::ser::Serializer as Ser;

use storage::parser::{FileHeaderParser, ParserError};
use storage::file::header::spec::FileHeaderSpec;
use storage::file::header::data::FileHeaderData;

pub struct JsonHeaderParser {
    spec: Option<FileHeaderSpec>,
}

impl JsonHeaderParser {

    pub fn new(spec: Option<FileHeaderSpec>) -> JsonHeaderParser {
        JsonHeaderParser {
            spec: spec
        }
    }

}

impl Display for JsonHeaderParser {

    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        try!(write!(fmt, "JsonHeaderParser"));
        Ok(())
    }

}

impl Debug for JsonHeaderParser {

    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        try!(write!(fmt, "JsonHeaderParser, Spec: {:?}", self.spec));
        Ok(())
    }

}

impl FileHeaderParser for JsonHeaderParser {

    fn read(&self, string: Option<String>)
        -> Result<FileHeaderData, ParserError>
    {
        if string.is_some() {
            let s = string.unwrap();
            debug!("Deserializing: {}", s);
            let fromstr : R<Value> = from_str(&s[..]);
            if let Ok(ref content) = fromstr {
                return Ok(visit_json(&content))
            }
            let oe = fromstr.err().unwrap();
            let s = format!("JSON parser error: {}", oe.description());
            let e = ParserError::short(&s[..], s.clone(), 0);
            Err(e)
        } else {
            Ok(FileHeaderData::Null)
        }
    }

    fn write(&self, data: &FileHeaderData) -> Result<String, ParserError> {
        let mut s = Vec::<u8>::new();
        {
            let mut ser = Serializer::pretty(&mut s);
            data.serialize(&mut ser).map_err(|e| {
                debug!("Serializer error: {:?}", e);
            }).ok();
        }

        String::from_utf8(s).or(
            Err(ParserError::short("Cannot parse utf8 bytes",
                                   String::from("<not printable>"),
                                   0)))
    }

}

// TODO: This function must be able to return a parser error
fn visit_json(v: &Value) -> FileHeaderData {
    match v {
        &Value::Null             => FileHeaderData::Null,
        &Value::Bool(b)          => FileHeaderData::Bool(b),
        &Value::I64(i)           => FileHeaderData::Integer(i),
        &Value::U64(u)           => FileHeaderData::UInteger(u),
        &Value::F64(f)           => FileHeaderData::Float(f),
        &Value::String(ref s)        => FileHeaderData::Text(s.clone()),
        &Value::Array(ref vec)       => {
            FileHeaderData::Array {
                values: Box::new(vec.clone().into_iter().map(|i| visit_json(&i)).collect())
            }
        },
        &Value::Object(ref btree)    => {
            let btree = btree.clone();
            FileHeaderData::Map{
                keys: btree.into_iter().map(|(k, v)|
                    FileHeaderData::Key {
                        name: k,
                        value: Box::new(visit_json(&v)),
                    }
                ).collect()
            }
        }
    }
}

impl Serialize for FileHeaderData {

    fn serialize<S>(&self, ser: &mut S) -> Result<(), S::Error>
        where S: Ser
    {
        match self {
            &FileHeaderData::Null               => {
                let o : Option<bool> = None;
                o.serialize(ser)
            },
            &FileHeaderData::Bool(ref b)            => b.serialize(ser),
            &FileHeaderData::Integer(ref i)         => i.serialize(ser),
            &FileHeaderData::UInteger(ref u)        => u.serialize(ser),
            &FileHeaderData::Float(ref f)           => f.serialize(ser),
            &FileHeaderData::Text(ref s)            => (&s[..]).serialize(ser),
            &FileHeaderData::Array{values: ref vs}  => vs.serialize(ser),
            &FileHeaderData::Map{keys: ref ks}      => {
                let mut hm = HashMap::new();

                for key in ks {
                    if let &FileHeaderData::Key{name: ref n, value: ref v} = key {
                        hm.insert(n, v);
                    } else {
                        panic!("Not a key: {:?}", key);
                    }
                }

                hm.serialize(ser)
            },
            &FileHeaderData::Key{name: _, value: _} => unreachable!(),

        }
    }

}

#[cfg(test)]
mod test {

    use std::ops::Deref;

    use super::JsonHeaderParser;
    use storage::parser::FileHeaderParser;
    use storage::file::header::data::FileHeaderData as FHD;
    use storage::file::header::spec::FileHeaderSpec as FHS;

    #[test]
    fn test_deserialization() {
        let text = String::from("{\"a\": 1, \"b\": -2}");
        let spec = FHS::Map {
            keys: vec![
                FHS::Key {
                    name: String::from("a"),
                    value_type: Box::new(FHS::UInteger)
                },
                FHS::Key {
                    name: String::from("b"),
                    value_type: Box::new(FHS::Integer)
                }
            ]
        };

        let parser = JsonHeaderParser::new(Some(spec));
        let parsed = parser.read(Some(text));
        assert!(parsed.is_ok(), "Parsed is not ok: {:?}", parsed);

        match parsed.ok() {
            Some(FHD::Map{keys}) => {
                for k in keys {
                    match k {
                        FHD::Key{name, value} => {
                            assert!(name == "a" || name == "b", "Key unknown");
                            match value.deref() {
                                &FHD::UInteger(u) => assert_eq!(u, 1),
                                &FHD::Integer(i) => assert_eq!(i, -2),
                                _ => assert!(false, "Integers are not here"),
                            }
                        },
                        _ => assert!(false, "Key is not a Key"),
                    }
                }
            },

            _ => assert!(false, "Parsed is not a map"),
        }
    }

    #[test]
    fn test_deserialization_without_spec() {
        let text    = String::from("{\"a\": [1], \"b\": {\"c\": -2}}");
        let parser  = JsonHeaderParser::new(None);
        let parsed  = parser.read(Some(text));

        assert!(parsed.is_ok(), "Parsed is not ok: {:?}", parsed);

        match parsed.ok() {
            Some(FHD::Map{keys}) => {
                for k in keys {
                    match_key(&k);
                }
            },

            _ => assert!(false, "Parsed is not a map"),
        }
    }

    fn match_key(k: &FHD) {
        use std::ops::Deref;

        match k {
            &FHD::Key{ref name, ref value} => {
                assert!(name == "a" || name == "b", "Key unknown");
                match value.deref() {
                    &FHD::Array{ref values} => {
                        for value in values.iter() {
                            match value {
                                &FHD::UInteger(u) => assert_eq!(u, 1),
                                _ => assert!(false, "UInt is not an UInt"),
                            }
                        }
                    }

                    &FHD::Map{ref keys} => {
                        for key in keys.iter() {
                            match key {
                                &FHD::Key{ref name, ref value} => {
                                    match value.deref() {
                                        &FHD::Integer(i) => {
                                            assert_eq!(i, -2);
                                            assert_eq!(name, "c");
                                        },
                                        _ => assert!(false, "Int is not an Int"),
                                    };
                                },
                                _ => assert!(false, "Key is not a Key"),
                            }
                        }
                    }
                    _ => assert!(false, "Integers are not here"),
                }
            },
            _ => assert!(false, "Key in main Map is not a Key"),
        }
    }

    #[test]
    fn test_desser() {
        use serde_json::error::Result as R;
        use serde_json::{Value, from_str};

        let text    = String::from("{\"a\": [1], \"b\": {\"c\": -2}}");
        let parser  = JsonHeaderParser::new(None);

        let des = parser.read(Some(text.clone()));
        assert!(des.is_ok(), "Deserializing failed");

        let ser = parser.write(&des.unwrap());
        assert!(ser.is_ok(), "Parser error when serializing deserialized text");

        let json_text : R<Value> = from_str(&text[..]);
        let json_ser  : R<Value> = from_str(&ser.unwrap()[..]);

        assert!(json_text.is_ok(), "Could not use serde to serialize text for comparison");
        assert!(json_ser.is_ok(),  "Could not use serde to serialize serialized-deserialized text for comparison");
        assert_eq!(json_text.unwrap(), json_ser.unwrap());
    }

}
