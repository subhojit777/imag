#[macro_use] extern crate clap;
#[macro_use] extern crate log;
extern crate config;
extern crate regex;

use cli::CliConfig;
use configuration::Configuration;
use runtime::{ImagLogger, Runtime};
use clap::App;

mod cli;
mod configuration;
mod runtime;
mod module;
mod storage;

fn main() {
    let yaml = load_yaml!("../etc/cli.yml");
    let app = App::from_yaml(yaml);
    let mut config = CliConfig::new(app);
    let configuration = Configuration::new(&config);

    let logger = ImagLogger::init(&configuration, &config);
    debug!("Logger created!");

    let rt = Runtime::new(configuration, config);
    debug!("Runtime created!");

    info!("Hello, world!");
}
