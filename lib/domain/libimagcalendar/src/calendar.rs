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

use std::path::Path;
use std::io::Read;
use std::fs::OpenOptions;

use error::Result;
use error::CalendarError as CE;

use libimagstore::store::Entry;
use libimagentryref::reference::Ref;

use vobject::icalendar::ICalendar;

/// A Calendar is a set of calendar entries
///
/// A Calendar represents a ical file on the filesystem
pub trait Calendar : Ref {
    fn events(&self) -> Result<ICalendar>;
}

impl Calendar for Entry {

    fn events(&self) -> Result<ICalendar> {
        self.get_path()
            .map_err(CE::from)
            .and_then(readfile)
            .and_then(|s| ICalendar::build(&s).map_err(CE::from))
    }

}

fn readfile<A: AsRef<Path>>(p: A) -> Result<String> {
    let mut s = String::new();
    OpenOptions::new()
        .read(true)
        .write(false)
        .create(false)
        .open(p)?
        .read_to_string(&mut s)
        .map_err(CE::from)
        .map(|_| s)
}

