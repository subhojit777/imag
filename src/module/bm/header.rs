use storage::file::FileHeaderSpec as FHS;
use storage::file::FileHeaderData as FHD;

pub fn get_spec() -> FHS {
    FHS::Map { keys: vec![ url_key(), tags_key() ] }
}

fn url_key() -> FHS {
    FHS::Key { name: String::from("URL"), value_type: Box::new(FHS::Text) }
}

fn tags_key() -> FHS {
    FHS::Key { name: String::from("TAGS"), value_type: Box::new(text_array()) }
}

fn text_array() -> FHS {
    FHS::Array { allowed_types: vec![FHS::Text] }
}


pub fn build_header(url: &String, tags: &Vec<String>) -> FHD {
    FHD::Map {
        keys: vec![
            FHD::Key {
                name: String::from("URL"),
                value: Box::new(FHD::Text(url.clone()))
            },
            FHD::Key {
                name: String::from("TAGS"),
                value: Box::new(build_tag_array(tags))
            }
        ]
    }
}

fn build_tag_array(tags: &Vec<String>) -> FHD {
    let texttags = tags.into_iter().map(|t| FHD::Text(t.clone())).collect();
    FHD::Array { values: Box::new(texttags) }
}

