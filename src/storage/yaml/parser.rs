use std::fmt::{Debug, Display, Formatter};
use std::fmt;

use yaml_rust::Yaml;

use storage::parser::{FileHeaderParser, ParserError};
use storage::file::header::spec::FileHeaderSpec;
use storage::file::header::data::FileHeaderData;

pub struct YamlHeaderParser {
    spec: Option<FileHeaderSpec>,
}

impl YamlHeaderParser {

    pub fn new(spec: Option<FileHeaderSpec>) -> YamlHeaderParser {
        YamlHeaderParser {
            spec: spec
        }
    }

}

impl Display for YamlHeaderParser {

    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        try!(write!(fmt, "YamlHeaderParser"));
        Ok(())
    }

}

impl Debug for YamlHeaderParser {

    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        try!(write!(fmt, "YamlHeaderParser, Spec: {:?}", self.spec));
        Ok(())
    }

}

impl FileHeaderParser for YamlHeaderParser {

    fn read(&self, string: Option<String>) -> Result<FileHeaderData, ParserError> {
        use yaml_rust::YamlLoader;
        if string.is_some() {
            let s = string.unwrap();
            YamlLoader::load_from_str(&s[..])
                .map(|mut vec_yaml| {
                    vec_yaml.pop().map(|f| {
                        visit_yaml(f)
                    }).unwrap()
                })
                .map_err(|e| {
                    debug!("YAML parser error: {:?}", e);
                    ParserError::short(&s[..], s.clone(), 0)
                })
        } else {
            Ok(FileHeaderData::Null)
        }

    }

    fn write(&self, data: &FileHeaderData) -> Result<String, ParserError> {
        use yaml_rust::YamlEmitter;

        let mut buffer  = String::new();
        let result = {
            let mut emitter = YamlEmitter::new(&mut buffer);
            emitter.dump(&visit_header(data))
        };
        result
            .map_err(|e| {
                error!("Error emitting YAML.");
                debug!("YAML parser error: {:?}", e);
                ParserError::short(&buffer[..], buffer.clone(), 0)
            })
            .map(|_| buffer)
    }

}

fn visit_yaml(v: Yaml) -> FileHeaderData {
    use std::process::exit;

    match v {
        Yaml::Real(_)    => FileHeaderData::Float(v.as_f64().unwrap()),
        Yaml::Integer(i) => {
            if i > 0 {
                debug!("Castring {} : i64 -> u64", i);
                FileHeaderData::UInteger(i as u64)
            } else {
                FileHeaderData::Integer(i)
            }
        },
        Yaml::String(s)  => FileHeaderData::Text(s),
        Yaml::Boolean(b) => FileHeaderData::Bool(b),

        Yaml::Array(vec) => {
            FileHeaderData::Array {
                values: Box::new(vec.clone().into_iter().map(|i| visit_yaml(i)).collect())
            }
        },

        Yaml::Hash(btree) => {
            let btree = btree.clone();
            FileHeaderData::Map{
                keys: btree.into_iter().map(|(k, v)|
                    FileHeaderData::Key {
                        name: String::from(k.as_str().unwrap()),
                        value: Box::new(visit_yaml(v)),
                    }
                ).collect()
            }
        },

        Yaml::Alias(_) => {
            warn!("YAML::ALIAS is not yet fully supported by rust-yaml");
            FileHeaderData::Null
        },

        Yaml::Null => FileHeaderData::Null,

        Yaml::BadValue => {
            warn!("YAML parsing error");
            exit(1);
        },
    }
}

fn visit_header(h: &FileHeaderData) -> Yaml {
    use std::ops::Deref;
    use std::collections::BTreeMap;
    use std::process::exit;

    match h {
        &FileHeaderData::Null               => Yaml::Null,
        &FileHeaderData::Float(f)           => Yaml::Real(format!("{}", f)),
        &FileHeaderData::Integer(i)         => Yaml::Integer(i),
        &FileHeaderData::UInteger(u)        => {
            debug!("Might be losing data now: u64 -> i64 cast");
            Yaml::Integer(u as i64)
        },
        &FileHeaderData::Text(ref s)            => Yaml::String(s.clone()),
        &FileHeaderData::Bool(b)            => Yaml::Boolean(b),

        &FileHeaderData::Array{values: ref a} => {
            Yaml::Array(a.deref().into_iter().map(|e| visit_header(e)).collect())
        },

        &FileHeaderData::Key{name: _, value: _} => {
            error!("Something went terribly wrong when trying to emit YAML");
            exit(1);
        },

        &FileHeaderData::Map{ref keys} => {
            let mut map : BTreeMap<Yaml, Yaml> = BTreeMap::new();

            let failed = keys.into_iter().map(|key| {
                match key {
                    &FileHeaderData::Key{ref name, ref value} => {
                        let k = Yaml::String(name.clone());
                        let v = visit_header(value.deref());

                        map.insert(k, v).is_none()
                    },

                    _ =>  {
                        error!("Something went terribly wrong when trying to emit YAML");
                        exit(1);
                    }
                }
            })
            .fold(0, |acc, succeeded : bool| {
                if !succeeded { acc + 1 } else { acc }
            });

            debug!("Failed to insert {} keys", failed);
            Yaml::Hash(map)
        },
    }
}
