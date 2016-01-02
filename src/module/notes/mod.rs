use std::fmt::{Debug, Formatter};
use std::fmt::Result as FMTResult;
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;

use clap::ArgMatches;
use regex::Regex;

mod header;

use module::Module;
use runtime::Runtime;
use storage::file::File;
use storage::parser::Parser;
use storage::json::parser::JsonHeaderParser;
use module::helpers::cli::create_tag_filter;
use module::helpers::cli::create_hash_filter;
use module::helpers::cli::create_text_header_field_grep_filter;
use module::helpers::cli::create_content_grep_filter;
use module::helpers::cli::CliFileFilter;

pub struct Notes<'a> {
    rt: &'a Runtime<'a>,
}

impl<'a> Notes<'a> {

    pub fn new(rt: &'a Runtime<'a>) -> Notes<'a> {
        Notes {
            rt: rt,
        }
    }

    fn command_add(&self, matches: &ArgMatches) -> bool {
        use std::process::exit;
        use self::header::build_header;

        let parser = Parser::new(JsonHeaderParser::new(None));
        let name   = matches.value_of("name")
                            .map(String::from)
                            .unwrap_or(String::from(""));
        let tags   = matches.value_of("tags")
                            .and_then(|s| Some(s.split(",").map(String::from).collect()))
                            .unwrap_or(vec![]);

        debug!("Building header with");
        debug!("    name = '{:?}'", name);
        debug!("    tags = '{:?}'", tags);
        let header = build_header(name, tags);

        let fileid = self.rt.store().new_file_with_header(self, header);
        self.rt
            .store()
            .load(self, &parser, &fileid)
            .and_then(|file| {
                info!("Created file in memory: {}", fileid);
                Some(self.rt.store().persist(&parser, file))
            })
            .unwrap_or(false)
    }

    fn command_list(&self, matches: &ArgMatches) -> bool {
        use ui::file::{FilePrinter, TablePrinter};
        use self::header::get_name_from_header;
        use self::header::get_tags_from_header;
        use std::process::exit;
        use module::helpers::cli::CliFileFilter;

        let parser  = Parser::new(JsonHeaderParser::new(None));

        let filter = {
            let hash_filter = create_hash_filter(matches, "id", true);
            let head_filter = create_text_header_field_grep_filter(matches, "match", "NAME", true);
            let text_filter = create_content_grep_filter(matches, "match", true);
            let tags_filter = create_tag_filter(matches, "tags", true);
            hash_filter.or(Box::new(head_filter)).and(Box::new(text_filter)).and(Box::new(tags_filter))
        };

        let printer = TablePrinter::new(self.rt.is_verbose(), self.rt.is_debugging());

        printer.print_files_custom(
            self.rt.store()
                    .load_for_module(self, &parser)
                    .into_iter()
                    .filter(|f| filter.filter_file(f)),
            &|file| {
                let fl      = file.deref().borrow();
                let hdr     = fl.header();
                let name    = get_name_from_header(hdr);
                let tags    = get_tags_from_header(hdr);

                debug!("Custom printer field: name = '{:?}'", name);
                debug!("Custom printer field: tags = '{:?}'", tags);

                vec![name, tags.join(", ")]
            }
        );
        true
    }

    fn command_remove(&self, matches: &ArgMatches) -> bool {
        let parser = Parser::new(JsonHeaderParser::new(None));

        let filter = {
            let hash_filter = create_hash_filter(matches, "id", false);
            let text_filter = create_text_header_field_grep_filter(matches, "match", "URL", false);
            let tags_filter = create_tag_filter(matches, "tags", false);
            hash_filter.or(Box::new(text_filter)).or(Box::new(tags_filter))
        };

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
            .fold((0, 0), |acc, succeeded| {
                let (worked, failed) = acc;
                if succeeded {
                    (worked + 1, failed)
                } else {
                    (worked, failed + 1)
                }
            });

        let (worked, failed) = result;

        info!("Removing succeeded for {} files", worked);
        info!("Removing failed for {} files", failed);

        return failed == 0;
    }

    fn command_add_tags(&self, matches: &ArgMatches) -> bool {
        unimplemented!()
    }

    fn command_rm_tags(&self, matches: &ArgMatches) -> bool {
        unimplemented!()
    }

    fn command_set_tags(&self, matches: &ArgMatches) -> bool {
        unimplemented!()
    }

}

impl<'a> Module<'a> for Notes<'a> {

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

    fn name(&self) -> &'static str{
        "notes"
    }

}

impl<'a> Debug for Notes<'a> {

    fn fmt(&self, fmt: &mut Formatter) -> FMTResult {
        write!(fmt, "[Module][Notes]");
        Ok(())
    }

}
