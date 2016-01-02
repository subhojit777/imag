use std::fmt::{Debug, Formatter, Error};

extern crate log;
use log::{LogRecord, LogLevel, LogLevelFilter, LogMetadata, SetLoggerError};

pub use cli::CliConfig;
pub use configuration::Configuration as Cfg;

use storage::Store;

pub struct ImagLogger {
    lvl: LogLevel,
}

impl ImagLogger {

    pub fn new(lvl: LogLevel) -> ImagLogger {
        ImagLogger {
            lvl: lvl,
        }
    }

    pub fn init(config: &CliConfig) -> Result<(), SetLoggerError> {
        let lvl = if config.is_debugging() {
            LogLevelFilter::Debug
        } else if config.is_verbose() {
            LogLevelFilter::Info
        } else {
            LogLevelFilter::Error
        };

        log::set_logger(|max_log_lvl| {
            max_log_lvl.set(lvl);
            debug!("Init logger with: {}", lvl);
            Box::new(ImagLogger::new(lvl.to_log_level().unwrap()))
        })
    }

}

impl log::Log for ImagLogger {

    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= self.lvl
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            println!("[{}]: {}", record.level(), record.args());
        }
    }
}

/**
 * Runtime object, represents a single interface to both the CLI configuration and the
 * configuration file. Also carries the store object around and is basically an object which
 * contains everything which is required to run a command/module.
 */
pub struct Runtime<'a> {
    pub config : CliConfig<'a>,
    pub configuration : Cfg,
    pub store : Store,
}

impl<'a> Runtime<'a> {

    pub fn new(cfg: Cfg, config : CliConfig<'a>) -> Runtime<'a> {
        let sp = config.store_path().unwrap_or(cfg.store_path());
        Runtime {
            config: config,
            configuration: cfg,
            store: Store::new(sp),
        }
    }

    /**
     * Check whether we run verbose
     */
    pub fn is_verbose(&self) -> bool {
        self.config.is_verbose()
    }

    /**
     * Check whether we run in debugging
     */
    pub fn is_debugging(&self) -> bool {
        self.config.is_debugging()
    }

    /**
     * Get the store path we are currently using
     */
    pub fn store_path(&self) -> String {
        self.config.store_path().unwrap_or(self.configuration.store_path())
    }

    /**
     * Get the store object
     */
    pub fn store(&self) -> &Store {
        &self.store
    }

    /**
     * Get the runtime path we are currently using
     */
    pub fn get_rtp(&self) -> String {
        if let Some(rtp) = self.config.get_rtp() {
            rtp
        } else {
            self.configuration.get_rtp()
        }
    }

    pub fn editor(&self) -> String {
        use std::env::var;

        if let Some(editor) = self.config.editor() {
            editor + &self.config.editor_opts()[..]
        } else if let Some(editor) = self.configuration.editor() {
            editor + &self.configuration.editor_opts()[..]
        } else if let Ok(editor) = var("EDITOR") {
            editor
        } else {
            String::from("vim")
        }
    }

}

impl<'a> Debug for Runtime<'a> {

    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Runtime (verbose: {}, debugging: {}, rtp: {})",
            self.is_verbose(),
            self.is_debugging(),
            self.get_rtp())
    }

}

