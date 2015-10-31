use storage::file::FileHeaderSpec as FHS;

pub fn get_spec() -> FHS {
    FHS::Map {
        keys: vec![
            FHS::Key { name: "URL", value_type: Box::new(FHS::Text) },
            FHS::Key { name: "TAGS",
                  value_type: Box::new(FHS::Array {
                      allowed_types: vec![FHS::Text]
                  }),
            },
        ],
    }
}

