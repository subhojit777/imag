use std::fmt::{Debug, Display, Formatter};
use std::fmt;

use clap::ArgMatches;

use runtime::Runtime;
use module::Module;

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

    fn command_add(&self) -> bool {
        unimplemented!()
    }

    fn command_list(&self) -> bool {
        unimplemented!()
    }

    fn command_remove(&self) -> bool {
        unimplemented!()
    }


}

impl<'a> Module<'a> for BM<'a> {

    fn exec(&self, matches: &ArgMatches) -> bool {
        match matches.subcommand_name() {
            Some("add")     => self.command_add(),
            Some("list")    => self.command_list(),
            Some("remove")  => self.command_remove(),
            Some(_) | None  => {
                info!("No command given, doing nothing");
                false
            },
        }
    }

    fn name(&self) -> &'static str {
        "bm"
    }
}

impl<'a> Debug for BM<'a> {

    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "BM");
        Ok(())
    }

}

