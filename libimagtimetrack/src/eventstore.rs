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

//! Extension trait for libimagstore::store::Store
//!
//! This module contains traits and code for extending the Store with functions that can be used to
//! create, get and delete events.

use chrono::NaiveDateTime as NDT;
use toml::Value;
use toml_query::insert::TomlValueInsertExt;

use libimagstore::store::Store;
use libimagstore::store::FileLockEntry;
use libimagstore::storeid::StoreId;
use libimagentrydatetime::datepath::compiler::DatePathCompiler;

use result::Result;
use constants::*;
use error::TimeTrackErrorKind as TTEK;
use error::MapErrInto;
use iter::get::GetTimeTrackIter;

use tag::TimeTrackingTag as TTT;

pub trait TimeTrackStore<'a> {

    fn create_timetracking_now(&'a self, ts: &TTT)                     -> Result<FileLockEntry<'a>>;
    fn create_timetracking_at(&'a self, start: &NDT, ts: &TTT)         -> Result<FileLockEntry<'a>>;
    fn create_timetracking(&'a self, start: &NDT, end: &NDT, ts: &TTT) -> Result<FileLockEntry<'a>>;

    fn get_timetrackings<I>(&'a self) -> Result<GetTimeTrackIter<'a>>;
}

fn now() -> NDT {
    use chrono::offset::local::Local;
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
        COMPILER.compile(CRATE_NAME, start)
            .map_err_into(TTEK::StoreIdError)
            .and_then(|id| enhance_id_with_tag(id, ts))
            .and_then(|id| self.create(id).map_err_into(TTEK::StoreWriteError))
            .and_then(|mut fle| {
                let v = Value::String(ts.as_str().to_owned());
                fle.get_header_mut()
                    .insert(DATE_TIME_TAG_HEADER_PATH, v)
                    .map_err_into(TTEK::HeaderWriteError)
                    .map(|_| fle)
            })
            .and_then(|mut fle| {
                let v = Value::String(start.format(DATE_TIME_FORMAT).to_string());
                fle.get_header_mut()
                    .insert(DATE_TIME_START_HEADER_PATH, v)
                    .map_err_into(TTEK::HeaderWriteError)
                    .map(|_| fle)
            })
    }

    fn create_timetracking(&'a self, start: &NDT, end: &NDT, ts: &TTT) -> Result<FileLockEntry<'a>> {
        self.create_timetracking_at(start, ts)
            .and_then(|mut fle| {
                let v = Value::String(end.format(DATE_TIME_FORMAT).to_string());
                fle.get_header_mut()
                    .insert(DATE_TIME_END_HEADER_PATH, v)
                    .map_err_into(TTEK::HeaderWriteError)
                    .map(|_| fle)
            })
    }

    fn get_timetrackings<I>(&'a self) -> Result<GetTimeTrackIter<'a>> {
        self.retrieve_for_module(CRATE_NAME)
            .map_err_into(TTEK::StoreReadError)
            .map(|iter| GetTimeTrackIter::new(iter, self))
    }

}

/// TODO: We need a new function on StoreId to do this in a nice way:
///
/// `storeid.append_to_filename(string)`
///
fn enhance_id_with_tag(s: StoreId, t: &TTT) -> Result<StoreId> {
    let mut new = s.local().clone();
    new.push(t.as_str().to_owned());
    StoreId::new_baseless(new).map_err_into(TTEK::StoreIdError)
}

