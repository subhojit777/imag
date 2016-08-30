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

extern crate toml;
#[macro_use] extern crate log;
#[macro_use] extern crate semver;

#[macro_use] extern crate libimagstore;
#[macro_use] extern crate libimagerror;

module_entry_path_mod!("counter");

pub mod counter;
pub mod error;
pub mod result;

