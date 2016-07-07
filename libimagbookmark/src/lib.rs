#[macro_use] extern crate log;
extern crate semver;
extern crate url;
extern crate regex;

#[macro_use] extern crate libimagstore;
#[macro_use] extern crate libimagerror;
extern crate libimagentrylink;

module_entry_path_mod!("bookmark", "0.1.0");

pub mod collection;
pub mod error;
pub mod result;
