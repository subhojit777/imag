extern crate clap;

use cli::CliConfig;

use std::path::Path;
use config::reader::from_file;
use config::types::Config as Cfg;

use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Error;

pub struct Configuration {
    pub rtp         : String,
    pub store_sub   : String,
    pub verbose     : bool,
    pub debugging   : bool,
}

impl Configuration {

    pub fn new(config: &CliConfig) -> Configuration {
        use std::env::home_dir;

        let rtp = rtp_path(config).or(default_path());

        let mut verbose     = false;
        let mut debugging   = false;
        let mut store_sub   = String::from("/store");

        if let Some(cfg) = fetch_config(rtp.clone()) {
            if let Some(v) = cfg.lookup_boolean("verbose") {
                verbose = v;
            }
            if let Some(d) = cfg.lookup_boolean("debug") {
                debugging = d;
            }
            if let Some(s) = cfg.lookup_str("store") {
                store_sub = String::from(s);
            }
        }

        Configuration {
            verbose: verbose,
            debugging: debugging,
            store_sub: store_sub,
            rtp: rtp.unwrap_or(String::from("/tmp/")),
        }
    }

    pub fn is_verbose(&self) -> bool {
        self.verbose
    }

    pub fn is_debugging(&self) -> bool {
        self.debugging
    }

    pub fn store_path_str(&self) -> String {
        format!("{}{}", self.rtp, self.store_sub)
    }

    pub fn get_rtp(&self) -> String {
        self.rtp.clone()
    }

}

fn rtp_path(config: &CliConfig) -> Option<String> {
    config.cli_matches.value_of("rtp")
                      .and_then(|s| Some(String::from(s)))
}

fn fetch_config(rtp: Option<String>) -> Option<Cfg> {
    rtp.and_then(|r| from_file(Path::new(&(r.clone() + "/config"))).ok())
}

fn default_path() -> Option<String> {
    use std::env::home_dir;

    home_dir().and_then(|mut buf| {
        buf.push("/.imag");
        buf.to_str().map(|s| String::from(s))
    })

}

impl Debug for Configuration {

    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Configuration (verbose: {}, debugging: {}, rtp: {}, store path: {})",
            self.is_verbose(),
            self.is_debugging(),
            self.get_rtp(),
            self.store_path_str()
            )
    }

}

