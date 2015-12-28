use std::fmt::{Debug, Display, Formatter};
use std::fmt;

use clap::ArgMatches;

use runtime::Runtime;
use module::Module;

use storage::file::hash::FileHash;
use storage::file::id::FileID;
use storage::parser::FileHeaderParser;
use storage::parser::Parser;
use storage::json::parser::JsonHeaderParser;

mod header;

pub struct BM<'a> {
    rt: &'a Runtime<'a>,
}

impl<'a> BM<'a> {

    pub fn new(rt: &'a Runtime<'a>) -> BM<'a> {
        BM {
            rt: rt,
        }
    }

    fn runtime(&self) -> &Runtime {
        &self.rt
    }

    fn command_add(&self, matches: &ArgMatches) -> bool {
        use self::header::build_header;

        let parser = Parser::new(JsonHeaderParser::new(None));

        let url    = matches.value_of("url").map(String::from).unwrap(); // clap ensures this is present
        let tags   = matches.value_of("tags").and_then(|s| {
            Some(s.split(",").map(String::from).collect())
        }).unwrap_or(vec![]);

        debug!("Building header with");
        debug!("    url  = '{:?}'", url);
        debug!("    tags = '{:?}'", tags);
        let header = build_header(url, tags);

        let fileid = self.rt.store().new_file_with_header(self, header);
        self.rt.store().load(&fileid).and_then(|file| {
            info!("Created file in memory: {}", fileid);
            Some(self.rt.store().persist(&parser, file))
        }).unwrap_or(false)
    }

    fn command_list(&self, matches: &ArgMatches) -> bool {
        use ui::file::{FilePrinter, TablePrinter};
        use self::header::get_url_from_header;
        use self::header::get_tags_from_header;
        use std::ops::Deref;

        let parser = Parser::new(JsonHeaderParser::new(None));
        let files  = self.rt.store().load_for_module(self, &parser);
        let printer = TablePrinter::new(self.rt.is_verbose(), self.rt.is_debugging());

        printer.print_files_custom(files.into_iter(),
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

    fn command_remove(&self, matches: &ArgMatches) -> bool {
        use std::process::exit;

        let result =
            if matches.is_present("id") {
                debug!("Removing by ID (Hash)");
                let hash = FileHash::from(matches.value_of("id").unwrap());
                self.remove_by_hash(hash)
            } else if matches.is_present("tags") {
                debug!("Removing by tags");
                let tags = matches.value_of("tags")
                                  .unwrap()
                                  .split(",")
                                  .map(String::from)
                                  .collect::<Vec<String>>();
                self.remove_by_tags(tags)
            } else if matches.is_present("match") {
                debug!("Removing by match");
                self.remove_by_match(String::from(matches.value_of("match").unwrap()))
            } else {
                error!("Unexpected error. Exiting");
                exit(1);
                false
            };

        if result {
            info!("Removing succeeded");
        } else {
            info!("Removing failed");
        }

        return result;
    }

    fn remove_by_hash(&self, hash: FileHash) -> bool {
        use std::ops::Deref;

        debug!("Removing for hash = '{:?}'", hash);
        let parser = Parser::new(JsonHeaderParser::new(None));

        let file = self.rt.store().load_by_hash(self, &parser, hash);
        debug!("file = {:?}", file);
        file.map(|file| {
            debug!("File loaded, can remove now: {:?}", file);
            let f = file.deref().borrow();
            self.rt.store().remove(f.id().clone())
        }).unwrap_or(false)
    }

    fn remove_by_tags(&self, tags: Vec<String>) -> bool {
        use std::fs::remove_file;
        use std::ops::Deref;
        use self::header::get_tags_from_header;

        let parser = Parser::new(JsonHeaderParser::new(None));
        self.rt
            .store()
            .load_for_module(self, &parser)
            .iter()
            .filter(|file| {
                let f = file.deref().borrow();
                get_tags_from_header(f.header()).iter().any(|tag| {
                    tags.iter().any(|remtag| remtag == tag)
                })
            }).map(|file| {
                let f = file.deref().borrow();
                self.rt.store().remove(f.id().clone())
            }).all(|x| x)
    }

    fn remove_by_match(&self, matcher: String) -> bool {
        use self::header::get_url_from_header;
        use std::fs::remove_file;
        use std::ops::Deref;
        use std::process::exit;
        use regex::Regex;

        let re = Regex::new(&matcher[..]).unwrap_or_else(|e| {
            error!("Cannot build regex out of '{}'", matcher);
            error!("{}", e);
            exit(1);
        });

        debug!("Compiled '{}' to regex: '{:?}'", matcher, re);

        let parser = Parser::new(JsonHeaderParser::new(None));
        self.rt
            .store()
            .load_for_module(self, &parser)
            .iter()
            .filter(|file| {
                let f   = file.deref().borrow();
                let url = get_url_from_header(f.header());
                debug!("url = {:?}", url);
                url.map(|u| {
                    debug!("Matching '{}' ~= '{}'", re.as_str(), u);
                    re.is_match(&u[..])
                }).unwrap_or(false)
            }).map(|file| {
                let f = file.deref().borrow();
                self.rt.store().remove(f.id().clone())
            }).all(|x| x)
    }

}

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
        write!(fmt, "BM");
        Ok(())
    }

}

