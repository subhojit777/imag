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

#[macro_use] extern crate log;
extern crate semver;
extern crate toml;

extern crate libimagrt;
#[macro_use] extern crate libimagstore;
extern crate libimagentrytag;

module_entry_path_mod!("notes", "0.1.0");

pub mod error;
pub mod note;
pub mod result;

