pub mod header;
pub mod utils;

pub mod spec {
    use storage::file::header::spec::FileHeaderSpec as FHS;

    pub fn named_text(name: &str) -> FHS {
        FHS::Key { name: String::from(name), value_type: Box::new(FHS::Text) }
    }

    pub fn named_text_array(name: &str) -> FHS {
        FHS::Key { name: String::from(name), value_type: Box::new(text_array()) }
    }

    pub fn text_array() -> FHS {
        FHS::Array { allowed_types: vec![FHS::Text] }
    }

}
