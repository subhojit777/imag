extern crate clap;

use cli::Config;

use std::path::Path;
use clap::{App, ArgMatches};

pub struct Configuration {
    pub rtp : String,
}

impl Configuration {

    pub fn new(config: &Config) -> Configuration {
        Configuration {
            rtp: rtp_path(config),
        }
    }

}

fn rtp_path(config: &Config) -> String {
    String::from(config.cli_matches.value_of("rtp").unwrap_or("~/.imag/store/"))
}


