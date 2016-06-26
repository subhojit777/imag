#[macro_use] extern crate log;
#[macro_use] extern crate version;
extern crate semver;
extern crate clap;

extern crate libimagstore;
extern crate libimagrt;
extern crate libimagref;
extern crate libimagerror;
extern crate libimagentrylist;

mod ui;
use ui::build_ui;

fn main() {
    println!("Hello, world!");
}
