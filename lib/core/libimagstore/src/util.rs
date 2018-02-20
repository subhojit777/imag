//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015-2018 Matthias Beyer <mail@beyermatthias.de> and contributors
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

use std::fmt::Write;

use toml::Value;

use store::Result;
use store::Header;

#[cfg(feature = "early-panic")]
#[macro_export]
macro_rules! if_cfg_panic {
    ()                       => { panic!() };
    ($msg:expr)              => { panic!($msg) };
    ($fmt:expr, $($arg:tt)+) => { panic!($fmt, $($arg),+) };
}

#[cfg(not(feature = "early-panic"))]
#[macro_export]
macro_rules! if_cfg_panic {
    ()                       => { };
    ($msg:expr)              => { };
    ($fmt:expr, $($arg:tt)+) => { };
}

pub fn entry_buffer_to_header_content(buf: &str) -> Result<(Value, String)> {
    debug!("Building entry from string");
    let mut header          = String::new();
    let mut content         = String::new();
    let mut header_consumed = false;

    for line in buf.lines().skip(1) { // the first line is "---"
        if line == "---" {
            header_consumed = true;
            // do not further process the line
        } else {
            if !header_consumed {
                let _ = writeln!(header, "{}", line)?;
            } else {
                let _ = write!(content, "{}", line)?;
            }
        }
    }

    Ok((Value::parse(&header)?, String::from(content)))
}

