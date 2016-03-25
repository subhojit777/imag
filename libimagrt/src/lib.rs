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
#[macro_use] extern crate itertools;
#[cfg(unix)] extern crate xdg_basedir;
extern crate tempfile;

extern crate clap;
extern crate toml;

extern crate libimagstore;
extern crate libimagutil;

mod configuration;
mod logger;

pub mod edit;
pub mod error;
pub mod runtime;

