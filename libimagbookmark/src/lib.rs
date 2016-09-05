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
extern crate semver;
extern crate url;
extern crate regex;

#[macro_use] extern crate libimagstore;
#[macro_use] extern crate libimagerror;
extern crate libimagentrylink;

module_entry_path_mod!("bookmark");

pub mod collection;
pub mod error;
pub mod link;
pub mod result;
