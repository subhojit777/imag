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
extern crate crypto;
extern crate itertools;
extern crate semver;
extern crate toml;
extern crate version;
extern crate walkdir;

#[macro_use] extern crate libimagstore;
#[macro_use] extern crate libimagerror;
#[macro_use] extern crate libimagutil;
extern crate libimagentrylist;

module_entry_path_mod!("ref", "0.2.0");

pub mod error;
pub mod flags;
pub mod hasher;
pub mod hashers;
pub mod lister;
pub mod reference;
pub mod result;
