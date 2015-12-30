use std::fmt::{Debug, Formatter, Error};

extern crate clap;
use clap::{App, ArgMatches};

pub struct ModuleConfig {
    pub load : bool,
}

impl ModuleConfig {
    pub fn new() -> ModuleConfig {
        ModuleConfig {
            load: false,
        }
    }
}

pub struct CliConfig<'a> {
    pub module_configs  : Vec<ModuleConfig>,
    pub cli_matches     : ArgMatches<'a, 'a>,
}

impl<'a> CliConfig<'a> {
    pub fn new(app : clap::App<'a, 'a, 'a, 'a, 'a, 'a>) -> CliConfig<'a> {
        CliConfig {
            module_configs: vec![],
            cli_matches: app.get_matches(),
        }
    }

    /**
     * Check whether the CLI says we should run verbose
     */
    pub fn is_verbose(&self) -> bool {
        self.cli_matches.is_present("verbose") || self.is_debugging()
    }

    /**
     * Check whether the CLI says we should run in debugging
     */
    pub fn is_debugging(&self) -> bool {
        self.cli_matches.is_present("debug")
    }

    /**
     * Get the runtime path the CLI configured
     */
    pub fn get_rtp(&self) -> Option<String> {
        self.cli_matches.value_of("rtp").and_then(|s| Some(String::from(s)))
    }

    /**
     * Get the store path the CLI configured
     *
     * TODO: Implement properly. Not working by now.
     */
    pub fn store_path(&self) -> Option<String> {
        self.get_rtp().and_then(|rtp| {
            self.cli_matches
                .value_of("storepath")
                .and_then(|s| Some(rtp + s))
        })
    }

    pub fn editor(&self) -> Option<String> {
        self.cli_matches.value_of("editor").and_then(|s| Some(String::from(s)))
    }

    pub fn editor_opts(&self) -> String {
        self.cli_matches
            .value_of("editor_opts")
            .map(|s| String::from(s))
            .unwrap_or(String::from(""))
    }
}

impl<'a> Debug for CliConfig<'a> {

    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "CliConfig (verbose: {}, debugging: {}, rtp: {})",
            self.is_verbose(),
            self.is_debugging(),
            self.get_rtp().or(Some(String::from("NONE"))).unwrap())
    }

}
