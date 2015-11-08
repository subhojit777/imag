use serde_json::{Value, from_str};
use serde_json::error::Result as R;

use super::super::parser::{FileHeaderParser, ParserError};
use super::super::file::{FileHeaderSpec, FileHeaderData};


struct JsonHeaderParser<'a> {
    spec: &'a FileHeaderSpec,
}

impl<'a> FileHeaderParser<'a> for JsonHeaderParser<'a> {

    fn new(spec: &'a FileHeaderSpec) -> JsonHeaderParser<'a> {
        JsonHeaderParser {
            spec: spec
        }
    }

    fn read(&self, string: Option<String>)
        -> Result<FileHeaderData, ParserError>
    {
        if (string.is_some()) {
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
                values: Box::new(vec.clone().into_iter().map(|i| visit_json(v)).collect())
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
