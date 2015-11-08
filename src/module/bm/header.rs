use storage::file::FileHeaderSpec as FHS;
use storage::file::FileHeaderData as FHD;

pub fn get_spec() -> FHS {
    FHS::Map { keys: vec![ url_key(), tags_key() ] }
}

fn url_key() -> FHS {
    FHS::Key { name: "URL", value_type: Box::new(FHS::Text) }
}

fn tags_key() -> FHS {
    FHS::Key { name: "TAGS", value_type: Box::new(text_array()) }
}

fn text_array() -> FHS {
    FHS::Array { allowed_types: vec![FHS::Text] }
}


pub fn build_header(url: &String, tags: &Vec<String>) -> FHD {
    FHD::Map {
        keys: vec![
            FHD::Key {
                name: "URL",
                value: Box::new(FHD::Text(url.clone()))
            },
            FHD::Key {
                name: "TAGS",
                value: Box::new(FHD::Text(tags.connect(",")))
            }
        ]
    }
}

