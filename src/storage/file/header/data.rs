use regex::Regex;

#[derive(Debug)]
#[derive(Clone)]
pub enum FileHeaderData {
    Null,
    Bool(bool),
    Integer(i64),
    UInteger(u64),
    Float(f64),
    Text(String),
    Key { name: String, value: Box<FileHeaderData> },
    Map { keys: Vec<FileHeaderData> },
    Array { values: Box<Vec<FileHeaderData>> },
}

impl FileHeaderData {

    pub fn matches_with(&self, r: &Regex) -> bool {
        match self {
            &FileHeaderData::Text(ref t) => r.is_match(&t[..]),
            &FileHeaderData::Key{name: ref n, value: ref val} => {
                r.is_match(n) || val.matches_with(r)
            },

            &FileHeaderData::Map{keys: ref dks} => {
                dks.iter().any(|x| x.matches_with(r))
            },

            &FileHeaderData::Array{values: ref vs} => {
                vs.iter().any(|x| x.matches_with(r))
            }

            _ => false,
        }
    }
}
