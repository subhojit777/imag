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

extern crate semver;
extern crate uuid;
extern crate toml;
#[macro_use] extern crate log;
extern crate serde_json;

#[macro_use] extern crate libimagstore;
#[macro_use] extern crate libimagerror;
extern crate task_hookrs;

module_entry_path_mod!("todo");

pub mod error;
pub mod result;
pub mod task;

