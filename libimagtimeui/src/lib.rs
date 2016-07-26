#![deny(
    non_camel_case_types,
    non_snake_case,
    path_statements,
    trivial_numeric_casts,
    unstable_features,
    unused_allocation,
    unused_import_braces,
    unused_imports,
    unused_mut,
    unused_qualifications,
    while_true,
)]

extern crate chrono;
extern crate clap;
extern crate regex;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;

#[macro_use] extern crate libimagerror;

pub mod cli;
pub mod date;
pub mod datetime;
pub mod parse;
pub mod time;

