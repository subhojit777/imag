use std::fmt::{Debug, Formatter};
use std::fmt::Result as FMTResult;

mod header;

use module::Module;
use runtime::Runtime;

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
        unimplemented!()
    }

    fn command_list(&self, matches: &ArgMatches) -> bool {
        unimplemented!()
    }

    fn command_remove(&self, matches: &ArgMatches) -> bool {
        unimplemented!()
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
