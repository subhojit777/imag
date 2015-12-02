use serde_json::{Value, from_str};
use serde_json::error::Result as R;
use serde_json::Serializer;
use serde::ser::Serialize;
use serde::ser::Serializer as Ser;

use std::collections::HashMap;
use std::io::stdout;

use super::super::parser::{FileHeaderParser, ParserError};
use super::super::file::{FileHeaderSpec, FileHeaderData};


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

impl FileHeaderParser for JsonHeaderParser {

    fn read(&self, string: Option<String>)
        -> Result<FileHeaderData, ParserError>
    {
        if string.is_some() {
            let s = string.unwrap();
            debug!("Deserializing: {}", s);
            let fromstr : R<Value> = from_str(&s[..]);
            if let Ok(content) = fromstr {
                Ok(visit_json(&content))
            } else {
                Err(ParserError::short("Unknown JSON parser error", s.clone(), 0))
            }
        } else {
            Ok(FileHeaderData::Null)
        }
    }

    fn write(&self, data: &FileHeaderData) -> Result<String, ParserError> {
        let mut s = Vec::<u8>::new();
        {
            let mut ser = Serializer::pretty(&mut s);
            data.serialize(&mut ser);
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
            FileHeaderData::Map{
                keys: btree.clone().iter().map(|(k, v)|
                    FileHeaderData::Key {
                        name: k.clone(),
                        value: Box::new(visit_json(v)),
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
            &FileHeaderData::Map{keys: ref ks}      => ks.serialize(ser),
            &FileHeaderData::Key{name: ref n, value: ref v}   => {
                let mut hm = HashMap::new();
                hm.insert(n, v);
                hm.serialize(ser)
            }
        }
    }

}
