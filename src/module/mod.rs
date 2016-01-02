use std::fmt::Debug;

use clap::ArgMatches;

use runtime::Runtime;

pub mod bm;
pub mod helpers;
pub mod notes;

/**
 * Module interface, each module has to implement this.
 */
pub trait Module<'a> : Debug {
    fn exec(&self, matches: &ArgMatches) -> bool;
    fn name(&self) -> &'static str;

    fn runtime(&self) -> &Runtime;
}

