extern crate clap;

use cli::CliConfig;

use std::path::Path;
use config::reader::from_file;
use config::types::Config as Cfg;
use config::types::ScalarValue as S;

pub struct Configuration {
    pub rtp         : String,
    pub store_sub   : String,
    pub verbose     : bool,
    pub debugging   : bool,
}

impl Configuration {

    pub fn new(config: &CliConfig) -> Configuration {
        let rtp = rtp_path(config);

        let mut verbose     = false;
        let mut debugging   = false;
        let mut store_sub   = String::from("/store");

        if let Some(cfg) = fetch_config(&rtp) {
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
            rtp: rtp,
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

fn rtp_path(config: &CliConfig) -> String {
    String::from(config.cli_matches.value_of("rtp").unwrap_or("~/.imag/store/"))
}

fn fetch_config(rtp: &String) -> Option<Cfg> {
    from_file(Path::new(&(rtp.clone() + "/config"))).ok()
}

