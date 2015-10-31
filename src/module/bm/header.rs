use storage::file::FileHeaderSpec as FHS;

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

