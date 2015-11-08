use serde_json::Value;

use super::parser;

struct JsonHeaderParser {
    spec: &FileHeaderSpec,
}

impl FileHeaderParser for JsonHeaderParser {

    fn new(spec: &FileHeaderSpec) -> JsonHeaderParser {
        JsonHeaderParser {
            spec: spec
        }
    }

    fn read(&self, string: Option<String>)
        -> Result<FileHeaderData, ParserError>
    {
        if let Ok(content) = data = serde_json::from_str(&string[..]) {
        } else {
            ParserError::short("Unknown JSON parser error", string.clone(), 0)
        }
    }

    fn write(&self, data: &FileHeaderData) -> Result<String, ParserError> {
    }

}

