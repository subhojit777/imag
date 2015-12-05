use module::helpers::header as headerhelpers;
use storage::file::FileHeaderData as FHD;
use storage::file::FileHeaderSpec as FHS;

pub fn get_spec() -> FHS {
    FHS::Map {
        keys: vec![ headerhelpers::tags::spec::url_key(),
                    headerhelpers::tags::spec::tags_key() ]
    }
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
                value: Box::new(headerhelpers::tags::data::build_tag_array(tags))
            }
        ]
    }
}

pub fn get_tags_from_header(header: &FHD) -> Vec<String> {
    headerhelpers::tags::data::get_tags_from_header(header)
}

