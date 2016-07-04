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

extern crate clap;
extern crate itertools;
#[macro_use] extern crate log;
extern crate regex;
extern crate toml;

extern crate libimagstore;
#[macro_use] extern crate libimagerror;
#[macro_use] extern crate libimagutil;

pub mod error;
pub mod exec;
pub mod result;
pub mod tag;
pub mod tagable;
pub mod util;
pub mod ui;

