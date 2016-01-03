#[macro_use] extern crate clap;
#[macro_use] extern crate log;
#[macro_use] extern crate serde;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate glob;
#[macro_use] extern crate uuid;
#[macro_use] extern crate regex;
#[macro_use] extern crate prettytable;
extern crate hoedown;
extern crate url;
extern crate config;
extern crate open;
extern crate itertools;
extern crate ansi_term;

pub use cli::CliConfig;
pub use configuration::Configuration;
pub use runtime::{ImagLogger, Runtime};
pub use clap::App;
pub use module::Module;

pub mod cli;
pub mod configuration;
pub mod runtime;
pub mod module;
pub mod storage;
pub mod ui;
pub mod util;

pub use module::bm::BM;
pub use module::notes::Notes;

fn main() {
    use ansi_term::Colour::Yellow;

    let yaml          = load_yaml!("../etc/cli.yml");
    let app           = App::from_yaml(yaml);
    let config        = CliConfig::new(app);

    ImagLogger::init(&config);

    let configuration = Configuration::new(&config);

    debug!("Logger created!");
    debug!("CliConfig    : {:?}", &config);
    debug!("Configuration: {:?}", &configuration);

    let rt = Runtime::new(configuration, config);

    debug!("Runtime      : {:?}", &rt);

    let res = match rt.config.cli_matches.subcommand_name() {
        Some("bm")    => BM::new(&rt).exec(rt.config.cli_matches.subcommand_matches("bm").unwrap()),
        Some("notes") => Notes::new(&rt).exec(rt.config.cli_matches.subcommand_matches("notes").unwrap()),
        _             => false,
    };

    info!("{}", Yellow.paint(format!("Module execution ended with {}", res)));
}
