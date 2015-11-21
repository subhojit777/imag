#![feature(box_patterns)]

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

use cli::CliConfig;
use configuration::Configuration;
use runtime::{ImagLogger, Runtime};
use clap::App;
use module::Module;
use module::ModuleError;
use module::CommandEnv;
use module::bm::BMModule;
use storage::backend::StorageBackend;

mod cli;
mod configuration;
mod runtime;
mod module;
mod storage;
mod ui;

use std::process::exit;

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

    let backend = StorageBackend::new(&rt).unwrap_or_else(|e| {
        error!("Error: {}", e);
        exit(1);
    });

    if let Some(matches) = rt.config.cli_matches.subcommand_matches("bm") {
        let module            = BMModule::new(&rt);
        let commands          = module.get_commands(&rt);
        if let Some(command)  = matches.subcommand_name() {
            debug!("Subcommand: {}", command);

            let cmdenv = CommandEnv {
                rt: &rt,
                bk: &backend,
                matches: matches.subcommand_matches(command).unwrap(),
            };

            let result = match commands.get(command) {
                Some(f) => f(&module, cmdenv),
                None    => Err(ModuleError::new("No subcommand found")),
            };

            debug!("Result of command: {:?}", result);
        } else {
            debug!("No subcommand");
        }

        module.shutdown(&rt);
    } else {
        // Err(ModuleError::mk("No commandline call"))
        info!("No commandline call...")
    }


    info!("Hello, world!");
}
