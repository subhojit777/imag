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
extern crate crossbeam;
extern crate hoedown;
extern crate url;
extern crate libimagstore;
#[macro_use] extern crate libimagerror;

pub mod error;
pub mod html;
pub mod link;
pub mod result;

