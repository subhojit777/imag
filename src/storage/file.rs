#[derive(Debug)]
pub enum FileHeaderSpec {
    Null,
    Bool,
    Integer,
    UInteger,
    Float,
    Text,
    Key { name: String, value_type: Box<FileHeaderSpec> },
    Array { allowed_types: Box<Vec<FileHeaderSpec>> },
}

#[derive(Debug)]
pub enum FileHeaderData {
    Null,
    Bool(bool),
    Integer(i64),
    UInteger(u64),
    Float(f64),
    Text(String),
    Key { name: String, value: Box<FileHeaderData> },
    Array { values: Box<Vec<FileHeaderData>> },
}

pub trait FileData : Sized {
    fn get_fulltext(&self) -> String;
    fn get_abbrev(&self) -> String;
}

