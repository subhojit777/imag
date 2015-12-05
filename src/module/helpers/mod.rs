/*!
 * Utility helpers for modules
 */

pub mod cli;
pub mod header;
pub mod utils;
pub mod content;

/**
 * Helpers for header specs
 */
pub mod spec {
    use storage::file::header::spec::FileHeaderSpec as FHS;

    /**
     * Helper to get a spec for a Key-Value for a named text:
     *
     *  { '<name>': "<Text>" }
     */
    pub fn named_text(name: &str) -> FHS {
        FHS::Key { name: String::from(name), value_type: Box::new(FHS::Text) }
    }

    /**
     * Helper to get a spec for a Key-Value for a named array:
     *
     *  { '<name>': [ "<Text>", ...] }
     */
    pub fn named_text_array(name: &str) -> FHS {
        FHS::Key { name: String::from(name), value_type: Box::new(text_array()) }
    }

    /**
     * Helper to get a spec for Array<Text>:
     *
     *  [ "<Text>", ...]
     */
    pub fn text_array() -> FHS {
        FHS::Array { allowed_types: vec![FHS::Text] }
    }

}

