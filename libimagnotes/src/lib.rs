#[macro_use] extern crate log;
extern crate semver;
extern crate toml;

extern crate libimagrt;
#[macro_use] extern crate libimagstore;
extern crate libimagentrytag;

module_entry_path_mod!("notes", "0.1.0");

pub mod error;
pub mod note;
pub mod result;

