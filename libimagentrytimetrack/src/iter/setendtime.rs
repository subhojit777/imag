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

use toml::Value;
use toml_query::insert::TomlValueInsertExt;
use chrono::naive::datetime::NaiveDateTime as NDT;

use constants::*;
use error::TimeTrackError as TTE;
use error::TimeTrackErrorKind as TTEK;
use error::MapErrInto;
use iter::create::CreateTimeTrackIter;

use libimagstore::store::FileLockEntry;

pub struct SetEndTimeIter<'a> {
    inner: CreateTimeTrackIter<'a>,
    datetime: NDT,
}

impl<'a> SetEndTimeIter<'a>
{
    pub fn new(inner: CreateTimeTrackIter<'a>, datetime: NDT) -> SetEndTimeIter<'a> {
        SetEndTimeIter {
            inner: inner,
            datetime: datetime,
        }
    }
}

impl<'a> Iterator for SetEndTimeIter<'a> {
    type Item = Result<FileLockEntry<'a>, TTE>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|res| {
                res.and_then(|mut fle| {
                    let v = Value::String(self.datetime.format(DATE_TIME_FORMAT).to_string());
                    fle.get_header_mut()
                        .insert(DATE_TIME_END_HEADER_PATH, v)
                        .map_err_into(TTEK::HeaderWriteError)
                        .map(|_| fle)
                })
            })
    }

}

