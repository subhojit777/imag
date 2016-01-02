use std::fmt::{Debug, Formatter, Error};
use std::path::Path;

use config::reader::from_file;
use config::types::Config as Cfg;
use cli::CliConfig;

/**
 * Configuration object which represents the configuration file.
 *
 * It gets passed a CliConfig object on ::new(), retreives some data from this one which is then
 * provided as default value to the callee if there is no value for it in the configuration.
 *
 * TODO: Setup is kinda ugly, as we re-use data from the CLI, which is the job of the Runtime
 * object later.
 */
pub struct Configuration {
    pub rtp         : String,
    pub store_sub   : String,
    pub verbose     : bool,
    pub debugging   : bool,
    pub editor      : Option<String>,
    pub editor_opts : String,
}

impl Configuration {

    pub fn new(config: &CliConfig) -> Configuration {
        let rtp = rtp_path(config).or(default_path()).unwrap_or(String::from("/tmp/"));


        let cfg = fetch_config(&rtp);

        let verbose     = cfg.lookup_boolean("verbose").unwrap_or(false);
        let debugging   = cfg.lookup_boolean("debug").unwrap_or(false);
        let store_sub   = String::from(cfg.lookup_str("store").unwrap_or("/store"));
        let editor      = cfg.lookup_str("editor").map(String::from);
        let editor_opts = String::from(cfg.lookup_str("editor-opts").unwrap_or(""));

        debug!("Building configuration");
        debug!("  - verbose    : {}", verbose);
        debug!("  - debugging  : {}", debugging);
        debug!("  - store sub  : {}", store_sub);
        debug!("  - runtimepath: {}", rtp);
        debug!("  - editor     : {:?}", editor);
        debug!("  - editor-opts: {}", editor_opts);

        Configuration {
            verbose: verbose,
            debugging: debugging,
            store_sub: store_sub,
            rtp: rtp,
            editor: editor,
            editor_opts: editor_opts,
        }
    }

    /**
     * Check whether the configuration says we should run verbose
     */
    pub fn is_verbose(&self) -> bool {
        self.verbose
    }

    /**
     * Check whether the configuration says we should run in debugging
     */
    pub fn is_debugging(&self) -> bool {
        self.debugging
    }

    /**
     * Get the store path the configuration configured
     */
    pub fn store_path(&self) -> String {
        format!("{}{}", self.rtp, self.store_sub)
    }

    /**
     * Get the runtime path the configuration configured
     */
    pub fn get_rtp(&self) -> String {
        self.rtp.clone()
    }

    pub fn editor(&self) -> Option<String> {
        self.editor.clone()
    }

    pub fn editor_opts(&self) -> &String {
        &self.editor_opts
    }

}

/**
 * Helper to get the runtimepath from the CLI
 */
fn rtp_path(config: &CliConfig) -> Option<String> {
    config.cli_matches.value_of("rtp")
                      .and_then(|s| Some(String::from(s)))
}

fn fetch_config(rtp: &String) -> Cfg {
    use std::process::exit;

    let configpath = format!("{}{}", rtp, "/config");
    from_file(Path::new(&configpath)).map_err(|e| {
        println!("Error loading config at '{}' -> {:?}", configpath, e);
        println!("Exiting now.");
        exit(1)
    }).unwrap()
}

/**
 * Default runtime path, if available.
 */
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
            self.store_path()
            )
    }

}

