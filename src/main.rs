#[macro_use] extern crate clap;
#[macro_use] extern crate log;
#[macro_use] extern crate serde;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate glob;
#[macro_use] extern crate uuid;
#[macro_use] extern crate regex;
#[macro_use] extern crate prettytable;
extern crate url;
extern crate config;

use std::process::exit;

use cli::CliConfig;
use configuration::Configuration;
use runtime::{ImagLogger, Runtime};
use clap::App;
use module::Module;

mod cli;
mod configuration;
mod runtime;
mod module;
mod storage;
mod ui;
mod util;

use module::bm::BM;

fn main() {
    let yaml = load_yaml!("../etc/cli.yml");
    let app = App::from_yaml(yaml);
    let config = CliConfig::new(app);
    let configuration = Configuration::new(&config);

    let logger = ImagLogger::init(&configuration, &config);
    debug!("Logger created!");

    debug!("CliConfig    : {:?}", &config);
    debug!("Configuration: {:?}", &configuration);

    let rt = Runtime::new(configuration, config);

    debug!("Runtime      : {:?}", &rt);

    if let Some(matches) = rt.config.cli_matches.subcommand_matches("bm") {
        let res = BM::new(&rt).exec(matches);
        info!("BM exited with {}", res);
    } else {
        info!("No commandline call...")
    }

    info!("Hello, world!");
}
