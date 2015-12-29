/*!
 * Helpers for headers
 */

pub mod tags;

/**
 * Utility helpers for header data
 */
pub mod data {
    use std::ops::Deref;
    use storage::file::header::data::FileHeaderData as FHD;

    /**
     * Get an URL from a header, whereas the header has to have the following format:
     *
     *   { ..., "URL": "<URL>", ... }
     *
     * Does no spec verification.
     */
    pub fn get_url_from_header(header: &FHD) -> Option<String> {
        match header {
            &FHD::Map{keys: ref ks} => {
                let mut keys : Vec<FHD> = ks.clone();
                keys.iter().find(|k| {
                    match k.deref() {
                        &FHD::Key{name: ref n, value: ref v} => n == "URL",
                        _ => false
                    }
                }).and_then(|urlkey| {
                    match urlkey.deref().clone() {
                        FHD::Key{name: _, value: ref v} => {
                            match v.deref().clone() {
                                FHD::Text(s) => Some(s),
                                _ => {
                                    warn!("Malformed Header Data: Expected Text, found non-Text");
                                    None
                                },
                            }
                        }
                        _ => {
                            warn!("Malformed Header Data: Expected Text, found non-Text");
                            None
                        },
                    }
                })
            },
            _ => {
                warn!("Malformed Header Data: Expected Map, found non-Map");
                None
            }
        }
    }

}

