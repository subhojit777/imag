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

extern crate itertools;
#[macro_use] extern crate log;
extern crate toml;
extern crate semver;
extern crate url;
extern crate crypto;

#[macro_use] extern crate libimagstore;
#[macro_use] extern crate libimagerror;
#[macro_use] extern crate libimagutil;

module_entry_path_mod!("links", "0.2.0");

pub mod error;
pub mod external;
pub mod internal;
pub mod result;

