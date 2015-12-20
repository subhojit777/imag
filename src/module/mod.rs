use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fmt::Result as FMTResult;
use std::result::Result;

use clap::ArgMatches;

use runtime::Runtime;

pub mod bm;
pub mod helpers;

pub trait Module : Debug {
    fn new(rt: &Runtime) -> Self;
    fn exec(&self, matches: &ArgMatches) -> bool;
}

