use std::fmt::{Debug, Formatter};
use std::fmt;
use std::ops::Deref;
use std::process::exit;

use clap::ArgMatches;

use runtime::Runtime;
use module::Module;

use storage::parser::FileHeaderParser;
use storage::parser::Parser;
use storage::json::parser::JsonHeaderParser;
use module::helpers::cli::get_file_filter_by_cli;
use module::helpers::cli::CliFileFilter;

mod header;

use self::header::get_url_from_header;
use self::header::get_tags_from_header;

pub struct BM<'a> {
    rt: &'a Runtime<'a>,
}

impl<'a> BM<'a> {

    pub fn new(rt: &'a Runtime<'a>) -> BM<'a> {
        BM {
            rt: rt,
        }
    }

    /**
     * Subcommand: add
     */
    fn command_add(&self, matches: &ArgMatches) -> bool {
        use std::process::exit;
        use self::header::build_header;

        let parser = Parser::new(JsonHeaderParser::new(None));

        let url    = matches.value_of("url").map(String::from).unwrap(); // clap ensures this is present

        if !self.validate_url(&url, &parser) {
            error!("URL validation failed, exiting.");
            exit(1);
        } else {
            debug!("Verification succeeded");
        }

        let tags   = matches.value_of("tags").and_then(|s| {
            Some(s.split(",").map(String::from).collect())
        }).unwrap_or(vec![]);

        debug!("Building header with");
        debug!("    url  = '{:?}'", url);
        debug!("    tags = '{:?}'", tags);
        let header = build_header(url, tags);

        let fileid = self.rt.store().new_file_with_header(self, header);
        self.rt.store().load(self, &parser, &fileid).and_then(|file| {
            info!("Created file in memory: {}", fileid);
            Some(self.rt.store().persist(&parser, file))
        }).unwrap_or(false)
    }

    fn validate_url<HP>(&self, url: &String, parser: &Parser<HP>) -> bool
        where HP: FileHeaderParser
    {
        use util::is_url;

        if !is_url(url) {
            error!("Url '{}' is not a valid URL. Will not store.", url);
            return false;
        }

        let is_in_store = self.rt
            .store()
            .load_for_module(self, parser)
            .iter()
            .any(|file| {
                let f = file.deref().borrow();
                get_url_from_header(f.header()).map(|url_in_store| {
                    &url_in_store == url
                }).unwrap_or(false)
            });

        if is_in_store {
            error!("URL '{}' seems to be in the store already", url);
            return false;
        }

        return true;
    }

    /**
     * Subcommand: list
     */
    fn command_list(&self, matches: &ArgMatches) -> bool {
        use ui::file::{FilePrinter, TablePrinter};
        use std::ops::Deref;

        let parser = Parser::new(JsonHeaderParser::new(None));
        let filter : Box<CliFileFilter> = get_file_filter_by_cli(&parser, matches, "id", "match", "tags", Some("URL"));

        let files  = self.rt
            .store()
            .load_for_module(self, &parser)
            .into_iter()
            .filter(|file| filter.filter_file(file));
        let printer = TablePrinter::new(self.rt.is_verbose(), self.rt.is_debugging());

        printer.print_files_custom(files,
            &|file| {
                let fl = file.deref().borrow();
                let hdr = fl.header();
                let url = get_url_from_header(hdr).unwrap_or(String::from("Parser error"));
                let tags = get_tags_from_header(hdr);

                debug!("Custom printer field: url  = '{:?}'", url);
                debug!("Custom printer field: tags = '{:?}'", tags);

                vec![url, tags.join(", ")]
            }
        );
        true
    }

    /**
     * Subcommand: remove
     */
    fn command_remove(&self, matches: &ArgMatches) -> bool {
        let parser = Parser::new(JsonHeaderParser::new(None));
        let filter : Box<CliFileFilter> = get_file_filter_by_cli(&parser, matches, "id", "match", "tags", None);
        let result = self.rt
            .store()
            .load_for_module(self, &parser)
            .iter()
            .filter(|file| filter.filter_file(file))
            .map(|file| {
                debug!("File loaded, can remove now: {:?}", file);
                let f = file.deref().borrow();
                self.rt.store().remove(f.id().clone())
            })
            .all(|x| x);

        if result {
            info!("Removing succeeded");
        } else {
            info!("Removing failed");
        }

        return result;
    }

    /**
     * Subcommand: add_tags
     */
    fn command_add_tags(&self, matches: &ArgMatches) -> bool {
        self.alter_tags_in_files(matches, |old_tags, cli_tags| {
            let mut new_tags = old_tags.clone();
            new_tags.append(&mut cli_tags.clone());
            new_tags
        })
    }

    /**
     * Subcommand: rm_tags
     */
    fn command_rm_tags(&self, matches: &ArgMatches) -> bool {
        self.alter_tags_in_files(matches, |old_tags, cli_tags| {
            old_tags.clone()
                .into_iter()
                .filter(|tag| !cli_tags.contains(tag))
                .collect()
        })
    }

    /**
     * Subcommand: set_tags
     */
    fn command_set_tags(&self, matches: &ArgMatches) -> bool {
        self.alter_tags_in_files(matches, |old_tags, cli_tags| {
            cli_tags.clone()
        })
    }

    /**
     * Helper function to alter the tags in a file
     */
    fn alter_tags_in_files<F>(&self, matches: &ArgMatches, generate_new_tags: F) -> bool
        where F: Fn(Vec<String>, &Vec<String>) -> Vec<String>
    {
        use self::header::rebuild_header_with_tags;

        let cli_tags = matches.value_of("tags")
                          .map(|ts| {
                            ts.split(",")
                              .map(String::from)
                              .collect::<Vec<String>>()
                          })
                          .unwrap_or(vec![]);

        let parser = Parser::new(JsonHeaderParser::new(None));
        let filter : Box<CliFileFilter> = get_file_filter_by_cli(&parser,
                                                                 matches,
                                                                 "with_id",
                                                                 "with_match",
                                                                 "with_tags",
                                                                 None);
        self.rt
            .store()
            .load_for_module(self, &parser)
            .into_iter()
            .filter(|file| filter.filter_file(file))
            .map(|file| {
                debug!("Remove tags from file: {:?}", file);

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

                let new_header = rebuild_header_with_tags(&hdr, new_tags)
                    .unwrap_or_else(|| {
                        error!("Could not rebuild header for file");
                        exit(1);
                    });
                {
                    let mut f_mut = file.deref().borrow_mut();
                    f_mut.set_header(new_header);
                }

                self.rt.store().persist(&parser, file);
                true
            })
            .all(|x| x)
    }

}

/**
 * Trait implementation for BM module
 */
impl<'a> Module<'a> for BM<'a> {

    fn exec(&self, matches: &ArgMatches) -> bool {
        match matches.subcommand_name() {
            Some("add") => {
                self.command_add(matches.subcommand_matches("add").unwrap())
            },

            Some("list") => {
                self.command_list(matches.subcommand_matches("list").unwrap())
            },

            Some("remove") => {
                self.command_remove(matches.subcommand_matches("remove").unwrap())
            },

            Some("add_tags") => {
                self.command_add_tags(matches.subcommand_matches("add_tags").unwrap())
            },

            Some("rm_tags") => {
                self.command_rm_tags(matches.subcommand_matches("rm_tags").unwrap())
            },

            Some("set_tags") => {
                self.command_set_tags(matches.subcommand_matches("set_tags").unwrap())
            },

            Some(_) | None => {
                info!("No command given, doing nothing");
                false
            },
        }
    }

    fn name(&self) -> &'static str {
        "bookmark"
    }
}

impl<'a> Debug for BM<'a> {

    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        try!(write!(fmt, "BM"));
        Ok(())
    }

}

