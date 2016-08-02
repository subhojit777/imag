#![deny(
    dead_code,
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

extern crate clap;
#[macro_use] extern crate log;
extern crate toml;
extern crate prettytable;

extern crate libimagstore;
extern crate libimagutil;
#[macro_use] extern crate libimagerror;

pub mod cli;
pub mod error;
pub mod lister;
pub mod listers;
pub mod result;

