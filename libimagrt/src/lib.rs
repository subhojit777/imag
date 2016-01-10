#[macro_use] extern crate log;
#[macro_use] extern crate itertools;
#[cfg(unix)] extern crate xdg_basedir;

extern crate clap;
extern crate config;


extern crate libimagstore;
extern crate libimagutil;

mod configuration;
mod logger;

use std::path::PathBuf;

pub use clap::App;
use log::LogLevelFilter;

use clap::{Arg, ArgMatches};
use configuration::Configuration;
use logger::ImagLogger;
use libimagstore::store::Store;

pub struct Runtime<'a> {
    rtp: PathBuf,
    configuration: Configuration,
    cli_matches: ArgMatches<'a, 'a>,
    store: Store,
}

impl<'a> Runtime<'a> {

    /**
     * Gets the CLI spec for the program and retreives the config file path (or uses the default on
     * in $HOME/.imag/config, $XDG_CONFIG_DIR/imag/config or from env("$IMAG_CONFIG")
     * and builds the Runtime object with it.
     *
     * The cli_spec object should be initially build with the ::get_default_cli_builder() function.
     *
     */
    pub fn new(cli_spec: App<'a, 'a, 'a, 'a, 'a, 'a>) -> Runtime<'a> {
        let matches = cli_spec.get_matches();
        let rtp : PathBuf = matches.value_of("runtimepath").unwrap_or("~/.imag/").into();
        let storepath = matches.value_of("storepath")
                                .map(PathBuf::from)
                                .unwrap_or({
                                    let mut spath = rtp.clone();
                                    spath.push("/store");
                                    spath
                                });
        Runtime {
            cli_matches: matches,
            configuration: Configuration::new(&rtp).unwrap_or(Configuration::default()),
            rtp: rtp,
            store: Store::new(storepath),
        }
    }

    /**
     * appname should be "imag-foo"
     */
    pub fn get_default_cli_builder(appname: &'a str,
                                   version: &'a str,
                                   about: &'a str)
        -> App<'a, 'a, 'a, 'a, 'a, 'a>
    {
        App::new(appname)
            .version(version)
            .author("Matthias Beyer <mail@beyermatthias.de>")
            .about(about)
            .arg(Arg::with_name("verbosity")
                .short("v")
                .long("verbose")
                .help("Enables verbosity")
                .required(false)
                .takes_value(false))

            .arg(Arg::with_name("debugging")
                .long("debug")
                .help("Enables debugging output")
                .required(false)
                .takes_value(false))

            .arg(Arg::with_name("config")
                .short("c")
                .long("config")
                .help("Path to alternative config file")
                .required(false)
                .takes_value(true))

            .arg(Arg::with_name("runtimepath")
                .short("r")
                .long("rtp")
                .help("Alternative runtimepath")
                .required(false)
                .takes_value(true))

            .arg(Arg::with_name("storepath")
                .long("store")
                .help("Alternative storepath. Must be specified as full path, can be outside of the RTP")
                .required(false)
                .takes_value(true))
    }

    pub fn init_logger(&self) {
        let lvl = if self.is_debugging() {
            LogLevelFilter::Debug
        } else if self.is_verbose() {
            LogLevelFilter::Info
        } else {
            LogLevelFilter::Error
        };

        log::set_logger(|max_log_lvl| {
            max_log_lvl.set(lvl);
            debug!("Init logger with {}", lvl);
            Box::new(ImagLogger::new(lvl.to_log_level().unwrap()))
        })
        .map_err(|_| {
            panic!("Could not setup logger");
        })
        .ok();
    }

    pub fn is_verbose(&self) -> bool {
        self.cli_matches.is_present("verbosity")
    }

    pub fn is_debugging(&self) -> bool {
        self.cli_matches.is_present("debugging")
    }

    pub fn rtp(&self) -> &PathBuf {
        &self.rtp
    }

    pub fn cli(&self) -> &ArgMatches {
        &self.cli_matches
    }

    pub fn store(&self) -> &Store {
        &self.store
    }

}

