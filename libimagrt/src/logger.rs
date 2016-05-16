use std::io::Write;
use std::io::stderr;

use log::{Log, LogLevel, LogRecord, LogMetadata};

pub struct ImagLogger {
    lvl: LogLevel,
}

impl ImagLogger {

    pub fn new(lvl: LogLevel) -> ImagLogger {
        ImagLogger {
            lvl: lvl,
        }
    }

}

impl Log for ImagLogger {

    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= self.lvl
    }

    fn log(&self, record: &LogRecord) {
        use ansi_term::Colour::Red;
        use ansi_term::Colour::Yellow;
        use ansi_term::Colour::Cyan;

        if self.enabled(record.metadata()) {
            // TODO: This is just simple logging. Maybe we can enhance this lateron
            let loc = record.location();
            match record.metadata().level() {
                LogLevel::Debug => {
                    let lvl  = Cyan.paint(format!("{}", record.level()));
                    let file = Cyan.paint(format!("{}", loc.file()));
                    let ln   = Cyan.paint(format!("{}", loc.line()));
                    let args = Cyan.paint(format!("{}", record.args()));

                    writeln!(stderr(), "[imag][{: <5}][{}][{: >5}]: {}", lvl, file, ln, args).ok();
                },
                LogLevel::Warn | LogLevel::Error => {
                    let lvl  = Red.blink().paint(format!("{}", record.level()));
                    let args = Red.paint(format!("{}", record.args()));

                    writeln!(stderr(), "[imag][{: <5}]: {}", lvl, args).ok();
                },
                LogLevel::Info => {
                    let lvl  = Yellow.paint(format!("{}", record.level()));
                    let args = Yellow.paint(format!("{}", record.args()));

                    writeln!(stderr(), "[imag][{: <5}]: {}", lvl, args).ok();
                },
                _ => {
                    writeln!(stderr(), "[imag][{: <5}]: {}", record.level(), record.args()).ok();
                },
            }
        }
    }
}

