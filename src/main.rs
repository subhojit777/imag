#[macro_use] extern crate clap;
#[macro_use] extern crate log;
#[macro_use] extern crate serde;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate glob;
#[macro_use] extern crate uuid;
#[macro_use] extern crate regex;
extern crate config;

use cli::CliConfig;
use configuration::Configuration;
use runtime::{ImagLogger, Runtime};
use clap::App;
use module::Module;
use module::ModuleError;
use module::bm::BMModule;

mod cli;
mod configuration;
mod runtime;
mod module;
mod storage;
mod ui;

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
        let module : BMModule = Module::new(&rt);
        let commands          = module.get_commands(&rt);
        if let Some(command)  = matches.subcommand_name() {
            debug!("Subcommand: {}", command);
            match commands.get(command) {
                Some(f) => f(&rt),
                None    => debug!("No command '{}' found", command),
            }
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
