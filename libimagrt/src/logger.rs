//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015, 2016 Matthias Beyer <mail@beyermatthias.de> and contributors
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; version
// 2.1 of the License.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
//

use std::io::Write;
use std::io::stderr;

use log::{Log, LogLevel, LogRecord, LogMetadata};

use ansi_term::Style;
use ansi_term::Colour;
use ansi_term::ANSIString;

pub struct ImagLogger {
    lvl: LogLevel,
    color_enabled: bool,
}

impl ImagLogger {

    pub fn new(lvl: LogLevel) -> ImagLogger {
        ImagLogger {
            lvl: lvl,
            color_enabled: true
        }
    }

    pub fn with_color(mut self, b: bool) -> ImagLogger {
        self.color_enabled = b;
        self
    }

    fn style_or_not(&self, c: Style, s: String) -> ANSIString {
        if self.color_enabled {
            c.paint(s)
        } else {
            ANSIString::from(s)
        }
    }

    fn color_or_not(&self, c: Colour, s: String) -> ANSIString {
        if self.color_enabled {
            c.paint(s)
        } else {
            ANSIString::from(s)
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
                    let lvl  = self.color_or_not(Cyan, format!("{}", record.level()));
                    let file = self.color_or_not(Cyan, format!("{}", loc.file()));
                    let ln   = self.color_or_not(Cyan, format!("{}", loc.line()));
                    let args = self.color_or_not(Cyan, format!("{}", record.args()));

                    writeln!(stderr(), "[imag][{: <5}][{}][{: >5}]: {}", lvl, file, ln, args).ok();
                },
                LogLevel::Warn | LogLevel::Error => {
                    let lvl  = self.style_or_not(Red.blink(), format!("{}", record.level()));
                    let args = self.color_or_not(Red, format!("{}", record.args()));

                    writeln!(stderr(), "[imag][{: <5}]: {}", lvl, args).ok();
                },
                LogLevel::Info => {
                    let lvl  = self.color_or_not(Yellow, format!("{}", record.level()));
                    let args = self.color_or_not(Yellow, format!("{}", record.args()));

                    writeln!(stderr(), "[imag][{: <5}]: {}", lvl, args).ok();
                },
                _ => {
                    writeln!(stderr(), "[imag][{: <5}]: {}", record.level(), record.args()).ok();
                },
            }
        }
    }
}

