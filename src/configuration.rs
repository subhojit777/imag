extern crate clap;

use cli::Config;

use std::path::Path;
use clap::{App, ArgMatches};
use config::reader::from_file;
use config::types::Config as Cfg;
use config::types::Value as V;
use config::types::ScalarValue as S;

pub struct Configuration {
    pub rtp         : String,
    pub verbose     : bool,
    pub debugging   : bool,
}

impl Configuration {

    pub fn new(config: &Config) -> Configuration {
        let rtp = rtp_path(config);

        let mut verbose     = false;
        let mut debugging   = false;

        if let Some(cfg) = fetch_config(&rtp) {
            if let Some(v) = cfg.lookup_boolean("verbose") {
                verbose = v;
            }
            if let Some(d) = cfg.lookup_boolean("debug") {
                debugging = d;
            }
        }

        Configuration {
            verbose: verbose,
            debugging: debugging,
            rtp: rtp,
        }
    }

    pub fn is_verbose(&self) -> bool {
        self.verbose
    }

    pub fn is_debugging(&self) -> bool {
        self.debugging
    }

}

fn rtp_path(config: &Config) -> String {
    String::from(config.cli_matches.value_of("rtp").unwrap_or("~/.imag/store/"))
}

fn fetch_config(rtp: &String) -> Option<Cfg> {
    from_file(Path::new(&(rtp.clone() + "/config"))).ok()
}

