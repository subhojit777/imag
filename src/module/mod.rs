use std::fmt::Debug;

use clap::ArgMatches;

pub mod bm;
pub mod helpers;
pub mod notes;

/**
 * Module interface, each module has to implement this.
 */
pub trait Module<'a> : Debug {
    fn exec(&self, matches: &ArgMatches) -> bool;
    fn name(&self) -> &'static str;
}

