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
extern crate log;
extern crate interactor;

extern crate libimagstore;
#[macro_use]
extern crate libimagerror;

pub mod ui;

