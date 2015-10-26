extern crate log;

pub use cli::Config;

use std::io::stderr;
use std::io::Write;
use log::{LogRecord, LogLevel, LogLevelFilter, LogMetadata, SetLoggerError};

pub struct ImagLogger {
    lvl: LogLevel,
}

impl ImagLogger {

    pub fn new(lvl: LogLevel) -> ImagLogger {
        ImagLogger {
            lvl: lvl,
        }
    }

    pub fn early() -> Result<(), SetLoggerError> {
        ImagLogger::init_logger(LogLevelFilter::Error)
    }

    pub fn init(config: &Config) -> Result<(), SetLoggerError> {
        if config.is_debugging() {
            ImagLogger::init_logger(LogLevelFilter::Debug)
        } else if config.is_verbose() {
            ImagLogger::init_logger(LogLevelFilter::Info)
        } else {
            ImagLogger::init_logger(LogLevelFilter::Error)
        }

    }

    fn init_logger(lvlflt : LogLevelFilter) -> Result<(), SetLoggerError> {
        log::set_logger(|max_log_lvl| {
            max_log_lvl.set(lvlflt);
            debug!("Init logger with: {}", lvlflt);
            Box::new(ImagLogger::new(lvlflt.to_log_level().unwrap()))
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
    pub config : Config<'a>,
}

impl<'a> Runtime<'a> {

    pub fn new(config : Config<'a>) -> Runtime<'a> {
        Runtime {
            config: config,
        }
    }

    pub fn is_verbose(&self) -> bool {
        self.config.is_verbose()
    }

    pub fn is_debugging(&self) -> bool {
        self.config.is_debugging()
    }

}
