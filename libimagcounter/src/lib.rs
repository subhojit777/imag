extern crate toml;
#[macro_use] extern crate log;
#[macro_use] extern crate semver;

#[macro_use] extern crate libimagstore;

module_entry_path_mod!("counter", "0.1.0");

pub mod counter;
pub mod error;
pub mod result;

