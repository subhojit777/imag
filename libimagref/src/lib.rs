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

module_entry_path_mod!("ref", "0.1.0");

pub mod reference;
pub mod flags;
pub mod error;
pub mod result;

