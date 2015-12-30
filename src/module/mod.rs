use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fmt::Result as FMTResult;
use std::result::Result;

use clap::ArgMatches;

use runtime::Runtime;

pub mod bm;
pub mod helpers;

/**
 * Module interface, each module has to implement this.
 */
pub trait Module<'a> : Debug {
    fn exec(&self, matches: &ArgMatches) -> bool;
    fn name(&self) -> &'static str;
}

