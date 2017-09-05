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

use regex::Regex;
use toml::Value;

use store::Result;
use store::Header;
use error::StoreErrorKind as SEK;
use error::StoreError as SE;

#[cfg(feature = "early-panic")]
#[macro_export]
macro_rules! if_cfg_panic {
    ()                       => { panic!() };
    ($msg:expr)              => { panic!($msg) };
    ($fmt:expr, $($arg:tt)+) => { panic!($fmt, $($($arg),*)) };
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
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?smx)
            ^---$
            (?P<header>.*) # Header
            ^---$\n
            (?P<content>.*) # Content
        ").unwrap();
    }

    let matches = match RE.captures(buf) {
        None    => return Err(SE::from_kind(SEK::MalformedEntry)),
        Some(s) => s,
    };

    let header = match matches.name("header") {
        None    => return Err(SE::from_kind(SEK::MalformedEntry)),
        Some(s) => s
    };

    let content = matches.name("content").map(|r| r.as_str()).unwrap_or("");

    Ok((try!(Value::parse(header.as_str())), String::from(content)))
}

