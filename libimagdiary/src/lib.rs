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

extern crate chrono;
#[macro_use] extern crate log;
#[macro_use] extern crate lazy_static;
extern crate semver;
extern crate toml;
extern crate regex;
extern crate itertools;

#[macro_use] extern crate libimagstore;
extern crate libimagutil;
#[macro_use] extern crate libimagerror;
extern crate libimagrt;

module_entry_path_mod!("diary", "0.1.0");

pub mod config;
pub mod error;
pub mod diaryid;
pub mod diary;
pub mod is_in_diary;
pub mod entry;
pub mod iter;
pub mod result;
