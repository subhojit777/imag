#[macro_use] extern crate log;
extern crate clap;
#[macro_use] extern crate semver;
extern crate toml;
#[macro_use] extern crate version;

extern crate libimaglink;
extern crate libimagrt;
extern crate libimagstore;
extern crate libimagutil;

use libimagstore::store::Store;

mod ui;

use ui::build_ui;

fn main() {
    println!("Hello, world!");
}

