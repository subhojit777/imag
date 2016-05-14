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
extern crate toml;
extern crate fs2;
extern crate git2;

extern crate libimagstore;
extern crate libimagentrylink;
#[macro_use] extern crate libimagerror;

pub mod debug;
pub mod flock;
pub mod linkverify;

