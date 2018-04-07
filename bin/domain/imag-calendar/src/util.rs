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

use std::ops::Deref;

use libimagstore::store::FileLockEntry;
use libimagcalendar::event::Event;
use libimagerror::trace::MapErrTrace;

use regex::Regex;
use chrono::NaiveDateTime;
use filters::filter::Filter;

pub struct PastFilter(bool, NaiveDateTime);

impl PastFilter {
    pub fn new(show_past: bool, today: NaiveDateTime) -> Self {
        PastFilter(show_past, today)
    }
}

impl<'a> Filter<FileLockEntry<'a>> for PastFilter {

    fn filter(&self, f: &FileLockEntry) -> bool {
        self.0 || f.get_end().map_err_trace_exit_unwrap(1) >= self.1
    }
}

pub struct GrepFilter(Regex);

impl GrepFilter {
    pub fn new(grep: Regex) -> Self {
        GrepFilter(grep)
    }
}

impl<'a> Filter<FileLockEntry<'a>> for GrepFilter {

    fn filter(&self, f: &FileLockEntry) -> bool {
        use libimagutil::date::datetime_to_string;

        if self.0.is_match(&f.get_start().map(|dt| datetime_to_string(&dt)).map_err_trace_exit_unwrap(1)) {
            return true
        }

        if self.0.is_match(&f.get_end().map(|dt| datetime_to_string(&dt)).map_err_trace_exit_unwrap(1)) {
            return true
        }

        if self.0.is_match(&Event::get_location(f.deref()).map_err_trace_exit_unwrap(1)) {
            return true
        }

        if f.get_categories().map_err_trace_exit_unwrap(1).iter().any(|c| self.0.is_match(&c)) {
            return true
        }

        if self.0.is_match(&f.get_description().map_err_trace_exit_unwrap(1)) {
            return true
        }

        if f.get_uid().map_err_trace_exit_unwrap(1).map(|s| self.0.is_match(&s)).unwrap_or(false) {
            return true
        }

        false
    }
}
