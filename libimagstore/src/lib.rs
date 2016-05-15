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

#[macro_use] extern crate log;
#[macro_use] extern crate version;
extern crate fs2;
extern crate glob;
#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate toml;
#[cfg(test)] extern crate tempdir;
extern crate semver;
extern crate crossbeam;
extern crate walkdir;

#[macro_use] extern crate libimagerror;

pub mod storeid;
pub mod error;
pub mod hook;
pub mod store;
mod configuration;
mod lazyfile;

