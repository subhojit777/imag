#[macro_use] extern crate clap;

use cli::Config;

mod cli;

fn main() {
    let mut config = Config::new();
    cli::configure(&mut config);
    println!("Hello, world!");
}
