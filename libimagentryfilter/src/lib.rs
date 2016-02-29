#[macro_use] extern crate log;

extern crate itertools;
extern crate regex;
extern crate toml;

extern crate libimagstore;
extern crate libimagtag;

// core functionality modules of the crate,
// these depend only on libimagstore

pub mod cli;
pub mod builtin;
pub mod filter;
pub mod ops;

// extended functionality of the crate
// these depend on other internal libraries than libimagstore and use the upper core modules for
// their functionality

pub mod tags;
