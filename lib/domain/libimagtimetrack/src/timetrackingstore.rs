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

//! Extension trait for libimagstore::store::Store
//!
//! This module contains traits and code for extending the Store with functions that can be used to
//! create, get and delete events.

use chrono::NaiveDateTime as NDT;
use toml::Value;
use toml_query::insert::TomlValueInsertExt;

use libimagstore::store::Store;
use libimagstore::store::FileLockEntry;
use libimagentrydatetime::datepath::compiler::DatePathCompiler;

use error::Result;
use constants::*;
use iter::get::TimeTrackingsGetIterator;

use tag::TimeTrackingTag as TTT;

pub trait TimeTrackStore<'a> {

    fn create_timetracking_now(&'a self, ts: &TTT)                     -> Result<FileLockEntry<'a>>;
    fn create_timetracking_at(&'a self, start: &NDT, ts: &TTT)         -> Result<FileLockEntry<'a>>;
    fn create_timetracking(&'a self, start: &NDT, end: &NDT, ts: &TTT) -> Result<FileLockEntry<'a>>;

    fn get_timetrackings(&'a self) -> Result<TimeTrackingsGetIterator<'a>>;
}

fn now() -> NDT {
    use chrono::offset::Local;
    Local::now().naive_local()
}

lazy_static! {
    static ref COMPILER: DatePathCompiler = {
        use libimagentrydatetime::datepath::accuracy::Accuracy;
        use libimagentrydatetime::datepath::format::Format;

        DatePathCompiler::new(Accuracy::Second, Format::ElementIsFolder)
    };
}

impl<'a> TimeTrackStore<'a> for Store {

    fn create_timetracking_now(&'a self, ts: &TTT) -> Result<FileLockEntry<'a>> {
        self.create_timetracking_at(&now(), ts)
    }

    fn create_timetracking_at(&'a self, start: &NDT, ts: &TTT) -> Result<FileLockEntry<'a>> {
        use std::path::PathBuf;

        COMPILER.compile(CRATE_NAME, start)
            .map_err(From::from)
            .map(|mut id| {
                id.local_push(PathBuf::from(ts.as_str()));
                id
            })
            .and_then(|id| self.create(id).map_err(From::from))
            .and_then(|mut fle| {
                let v = Value::String(ts.as_str().to_owned());
                fle.get_header_mut()
                    .insert(DATE_TIME_TAG_HEADER_PATH, v)
                    .map_err(From::from)
                    .map(|_| fle)
            })
            .and_then(|mut fle| {
                let v = Value::String(start.format(DATE_TIME_FORMAT).to_string());
                fle.get_header_mut()
                    .insert(DATE_TIME_START_HEADER_PATH, v)
                    .map_err(From::from)
                    .map(|_| fle)
            })
    }

    fn create_timetracking(&'a self, start: &NDT, end: &NDT, ts: &TTT) -> Result<FileLockEntry<'a>> {
        self.create_timetracking_at(start, ts)
            .and_then(|mut fle| {
                let v = Value::String(end.format(DATE_TIME_FORMAT).to_string());
                fle.get_header_mut()
                    .insert(DATE_TIME_END_HEADER_PATH, v)
                    .map_err(From::from)
                    .map(|_| fle)
            })
    }

    fn get_timetrackings(&'a self) -> Result<TimeTrackingsGetIterator<'a>> {
        Ok(TimeTrackingsGetIterator::new(self.entries()?, self))
    }

}

