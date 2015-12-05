use std::fmt::{Debug, Formatter, Error};

extern crate log;
use log::{LogRecord, LogLevel, LogLevelFilter, LogMetadata, SetLoggerError};

pub use cli::CliConfig;
pub use configuration::Configuration as Cfg;

pub struct ImagLogger {
    lvl: LogLevel,
}

impl ImagLogger {

    pub fn new(lvl: LogLevel) -> ImagLogger {
        ImagLogger {
            lvl: lvl,
        }
    }

    pub fn init(cfg: &Cfg, config: &CliConfig) -> Result<(), SetLoggerError> {
        let lvl = if config.is_debugging() || cfg.is_debugging() {
            LogLevelFilter::Debug
        } else if config.is_verbose() || cfg.is_debugging() {
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

pub struct Runtime<'a> {
    pub config : CliConfig<'a>,
    pub configuration : Cfg,
}

impl<'a> Runtime<'a> {

    pub fn new(cfg: Cfg, config : CliConfig<'a>) -> Runtime<'a> {
        Runtime {
            config: config,
            configuration: cfg,
        }
    }

    pub fn is_verbose(&self) -> bool {
        self.config.is_verbose() || self.configuration.is_verbose()
    }

    pub fn is_debugging(&self) -> bool {
        self.config.is_debugging() || self.configuration.is_verbose()
    }

    pub fn store_path(&self) -> String {
        self.config.store_path().unwrap_or(self.configuration.store_path())
    }

    pub fn get_rtp(&self) -> String {
        if let Some(rtp) = self.config.get_rtp() {
            rtp
        } else {
            self.configuration.get_rtp()
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

