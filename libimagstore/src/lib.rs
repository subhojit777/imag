#[macro_use] extern crate version;
extern crate fs2;
extern crate regex;
extern crate toml;
#[cfg(test)] extern crate tempdir;
extern crate semver;

pub mod storeid;
pub mod error;
pub mod store;
mod lazyfile;

