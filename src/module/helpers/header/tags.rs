/*!
 * Helpers for headers - Tags
 */

/**
 * Spec helpers for header-tags
 */
pub mod spec {
    use storage::file::header::spec::FileHeaderSpec as FHS;
    use module::helpers::spec::{named_text, named_text_array};

    /**
     * helper for a Header spec for
     *
     *  { "URL": "<Text>" }
     */
    pub fn url_key() -> FHS {
        named_text("URL")
    }

    /**
     * helper for a Header spec for
     *
     *  { "TAGS": [ "<Text>", ... ] }
     */
    pub fn tags_key() -> FHS {
        named_text_array("TAGS")
    }

}

/**
 * Data helpers for header-tags
 */
pub mod data {
    use std::ops::Deref;
    use storage::file::header::data::FileHeaderData as FHD;
    use module::Module;
    use clap::ArgMatches;
    use storage::parser::Parser;
    use storage::parser::FileHeaderParser;

    /**
     * Use a Vec<String> to build a Tag-Array:
     *
     *  [ "<Text>", ... ]
     */
    pub fn build_tag_array(tags: Vec<String>) -> FHD {
        let texttags = tags.into_iter().map(|t| FHD::Text(t.clone())).collect();
        FHD::Array { values: Box::new(texttags) }
    }

    /**
     * Fetch tags from a header, whereas the header looks like this:
     *
     *   { ...,
     *     "TAGS": [ "<Text>", ... ],
     *     ...
     *   }
     *
     * Does no spec verification.
     */
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
                        FHD::Key{name: ref name, value: ref v} => {
                            if name == "TAGS" {
                                return match_array(v)
                            }
                        },
                        _ => warn!("Malformed Header Data: Expected Key, found non-Key"),
                    }
                }
            },
            _ => warn!("Malformed Header Data: Expected Map, found non-Map"),
        }

        tags
    }

    /**
     * Helper function to alter the tags in a file
     */
    pub fn alter_tags_in_files<HP, F, R>(m: &Module,
                                     matches: &ArgMatches,
                                     parser: &Parser<HP>,
                                     generate_new_tags: F,
                                     rebuild_header: R) -> bool
        where HP: FileHeaderParser,
              F:  Fn(Vec<String>, &Vec<String>) -> Vec<String>,
              R:  Fn(&FHD, Vec<String>) -> Option<FHD>
    {
        use std::process::exit;
        use module::helpers::cli::create_tag_filter;
        use module::helpers::cli::create_hash_filter;
        use module::helpers::cli::create_text_header_field_grep_filter;
        use module::helpers::cli::create_content_grep_filter;
        use module::helpers::cli::CliFileFilter;

        let cli_tags = matches.value_of("tags")
                          .map(|ts| {
                            ts.split(",")
                              .map(String::from)
                              .collect::<Vec<String>>()
                          })
                          .unwrap_or(vec![]);

        let filter = {
            let hash_filter = create_hash_filter(matches, "with:id", false);
            let text_filter = create_text_header_field_grep_filter(matches, "with_match", "URL", false);
            let tags_filter = create_tag_filter(matches, "with_tags", false);
            hash_filter.or(Box::new(text_filter)).or(Box::new(tags_filter))
        };

        m.runtime()
            .store()
            .load_for_module(m, &parser)
            .into_iter()
            .filter(|file| filter.filter_file(file))
            .map(|file| {
                debug!("Alter tags in file: {:?}", file);

                let hdr = {
                    let f = file.deref().borrow();
                    f.header().clone()
                };

                debug!("Tags:...");
                let old_tags = get_tags_from_header(&hdr);
                debug!("    old_tags = {:?}", &old_tags);
                debug!("    cli_tags = {:?}", &cli_tags);

                let new_tags = generate_new_tags(old_tags, &cli_tags);
                debug!("    new_tags = {:?}", &new_tags);

                let new_header = rebuild_header(&hdr, new_tags)
                    .unwrap_or_else(|| {
                        error!("Could not rebuild header for file");
                        exit(1);
                    });
                {
                    let mut f_mut = file.deref().borrow_mut();
                    f_mut.set_header(new_header);
                }

                m.runtime().store().persist(&parser, file);
                true
            })
            .all(|x| x)
    }


}

