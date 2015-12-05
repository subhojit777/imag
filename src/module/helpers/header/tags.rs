/*
 * Helpers for headers - Tags
 */

pub mod spec {
    use storage::file::FileHeaderSpec as FHS;
    use module::helpers::spec::{named_text, named_text_array};

    pub fn url_key() -> FHS {
        named_text("URL")
    }

    pub fn tags_key() -> FHS {
        named_text_array("TAGS")
    }

}

pub mod data {
    use std::ops::Deref;
    use storage::file::FileHeaderData as FHD;

    pub fn build_tag_array(tags: &Vec<String>) -> FHD {
        let texttags = tags.into_iter().map(|t| FHD::Text(t.clone())).collect();
        FHD::Array { values: Box::new(texttags) }
    }

    pub fn get_tags_from_header(header: &FHD) -> Vec<String> {
        let mut tags : Vec<String> = vec![];

        fn match_array(a: &Box<FHD>) -> Vec<String> {
            let mut tags : Vec<String> = vec![];

            match a.deref() {
                &FHD::Array{values: ref vs} => {
                    let values : Vec<FHD> = vs.deref().clone();
                    for value in values {
                        match value {
                            FHD::Text(t) => tags.push(t),
                            _ => warn!("Malformed Header Data: Expected Text, found non-Text"),
                        }
                    }
                }
                _ => warn!("Malformed Header Data: Expected Array, found non-Array"),
            }

            tags
        }

        match header {
            &FHD::Map{keys: ref ks} => {
                let keys : Vec<FHD> = ks.clone();
                for key in keys {
                    match key {
                        FHD::Key{name: _, value: ref v} => return match_array(v),
                        _ => warn!("Malformed Header Data: Expected Key, found non-Key"),
                    }
                }
            },
            _ => warn!("Malformed Header Data: Expected Map, found non-Map"),
        }

        tags
    }


}

