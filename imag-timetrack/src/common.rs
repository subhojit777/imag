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

use filters::filter::Filter;

use libimagentrytimetrack::timetracking::TimeTracking;
use libimagentrytimetrack::tag::TimeTrackingTag;
use libimagstore::store::FileLockEntry;
use libimagerror::trace::trace_error;

/// Check whether an timetracking has an end time.
///
/// Trace, if TimeTracking::get_end_datetime() returns Err(_)
pub fn has_end_time(timetracking: &FileLockEntry) -> bool {
    match timetracking.get_end_datetime() {
        Ok(x)  => x.is_some(),
        Err(e) => {
            warn!("Error checking {} whether End-time is set", timetracking.get_location());
            trace_error(&e);
            false
        }
    }
}

/// Check whether an timetracking has one of the passed tags
///
/// Trace, if TimeTracking::get_end_datetime() returns Err(_)
pub struct HasTagFromList<'a> {
    list: &'a Vec<TimeTrackingTag>,
}

impl<'a> HasTagFromList<'a> {
    pub fn new(v: &'a Vec<TimeTrackingTag>) -> HasTagFromList<'a> {
        HasTagFromList {
            list: v
        }
    }
}

impl<'a, 'f> Filter<FileLockEntry<'f>> for HasTagFromList<'a> {
    fn filter(&self, tracking: &FileLockEntry) -> bool {
        tracking.get_timetrack_tag().map(|t| self.list.contains(&t)).unwrap_or(false)
    }
}

