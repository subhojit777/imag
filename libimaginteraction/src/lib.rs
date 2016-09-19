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

extern crate spinner;
extern crate interactor;
#[macro_use] extern crate log;
extern crate ansi_term;
#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate clap;

extern crate libimagentryfilter;
extern crate libimagstore;
#[macro_use] extern crate libimagutil;
#[macro_use] extern crate libimagerror;

pub mod ask;
pub mod error;
pub mod filter;
pub mod result;
pub mod ui;

