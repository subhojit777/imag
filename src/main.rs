#[macro_use] extern crate clap;

use cli::Config;
use runtime::Runtime;

mod cli;
mod runtime;

fn main() {
    let mut config = Config::new();
    cli::configure(&mut config);

    let rt = Runtime::new(config);

    println!("Hello, world!");
}
