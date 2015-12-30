/*
 * Lets talk about header data first.
 * We need:
 *
 * - tags
 * - name (not unique)
 *
 * So an header could look like this:
 *
 * ```json
 *  {
 *   'name': "kittennotes",
 *   'tags': ['foo', 'bar', 'baz'],
 *  }
 * ```
 *
 * Nothing more is required for the header, I guess
 *
 */

use module::helpers;
use module::helpers::header as headerhelpers;
use storage::file::header::data::FileHeaderData as FHD;
use storage::file::header::spec::FileHeaderSpec as FHS;

pub fn get_spec() -> FHS {
    FHS::Map { keys: vec![ helpers::spec::named_text("NAME"),
                           headerhelpers::tags::spec::tags_key() ] }
}


pub fn build_header(name: String, tags: Vec<String>) -> FHD {
    FHD::Map {
        keys: vec![
            FHD::Key {
                name: String::from("NAME"),
                value: Box::new(FHD::Text(name.clone()))
            },
            FHD::Key {
                name: String::from("TAGS"),
                value: Box::new(headerhelpers::tags::data::build_tag_array(tags))
            }
        ]
    }
}

pub fn get_tags_from_header(header: &FHD) -> Vec<String> {
    headerhelpers::tags::data::get_tags_from_header(header)
}

/**
 * Get the name from the Header
 *
 * Returns empty string if there is no NAME field
 */
pub fn get_name_from_header(header: &FHD) -> String {
    headerhelpers::data::get_name_from_header(header).unwrap_or(String::from(""))
}

