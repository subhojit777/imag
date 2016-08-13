#![deny(
    non_camel_case_types,
    non_snake_case,
    path_statements,
    trivial_numeric_casts,
    unstable_features,
    unused_allocation,
    unused_import_braces,
    unused_imports,
    unused_must_use,
    unused_mut,
    unused_qualifications,
    while_true,
)]

#[macro_use] extern crate log;

extern crate filters;
extern crate itertools;
extern crate regex;
extern crate semver;
extern crate toml;

extern crate libimagstore;
extern crate libimagentrytag;

// core functionality modules of the crate,
// these depend only on libimagstore

pub mod cli;
pub mod builtin;
pub mod ops;

// extended functionality of the crate
// these depend on other internal libraries than libimagstore and use the upper core modules for
// their functionality

pub mod tags;
