extern crate clap;
extern crate glob;
#[macro_use] extern crate log;
extern crate semver;
extern crate toml;
#[macro_use] extern crate version;

extern crate libimagrt;
extern crate libimagstore;
extern crate libimagutil;

use std::process::exit;

use libimagrt::runtime::Runtime;
use libimagstore::store::FileLockEntry;
use libimagutil::trace::trace_error;

mod ui;

use ui::build_ui;

fn main() {
    println!("Hello, world!");
}
