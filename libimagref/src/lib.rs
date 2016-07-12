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

module_entry_path_mod!("ref", "0.2.0");

pub mod error;
pub mod flags;
pub mod reference;
pub mod result;

