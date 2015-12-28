use std::fmt::{Debug, Display, Formatter};
use std::fmt;

use clap::ArgMatches;

use runtime::Runtime;
use module::Module;

use storage::parser::FileHeaderParser;
use storage::parser::Parser;
use storage::json::parser::JsonHeaderParser;

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
        unimplemented!()
    }

    fn command_remove(&self, matches: &ArgMatches) -> bool {
        unimplemented!()
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

