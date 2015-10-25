#[macro_use] extern crate clap;

use cli::Config;
use runtime::Runtime;
use clap::App;

mod cli;
mod runtime;
mod module;
mod storage;

fn main() {
    let yaml = load_yaml!("../etc/cli.yml");
    let app = App::from_yaml(yaml);
    let mut config = Config::new(app);

    let rt = Runtime::new(config);

    println!("Hello, world!");
}
