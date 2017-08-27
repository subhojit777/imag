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

use result::Result;

use chrono::NaiveDateTime;
use filters::filter::Filter;

use libimagstore::store::FileLockEntry;

use tag::TimeTrackingTag as TTT;
use timetracking::TimeTracking;


pub fn has_start_time(entry: &FileLockEntry) -> bool {
    is_match!(entry.get_start_datetime(), Ok(Some(_)))
}

pub fn has_end_time(entry: &FileLockEntry) -> bool {
    is_match!(entry.get_end_datetime(), Ok(Some(_)))
}

pub fn has_tag(entry: &FileLockEntry) -> bool {
    is_match!(entry.get_timetrack_tag(), Ok(_))
}

pub fn has_start_time_where<F>(f: F) -> HasStartTimeWhere<F>
    where F: Fn(&NaiveDateTime) -> bool
{
    HasStartTimeWhere::new(f)
}

pub fn has_end_time_where<F>(f: F) -> HasEndTimeWhere<F>
    where F: Fn(&NaiveDateTime) -> bool
{
    HasEndTimeWhere::new(f)
}

pub fn has_one_of_tags<'a>(tags: &'a Vec<TTT>) -> HasOneOfTags<'a> {
    HasOneOfTags::new(tags)
}

mod types {
    use chrono::NaiveDateTime;
    use filters::filter::Filter;

    use tag::TimeTrackingTag as TTT;
    use timetracking::TimeTracking;

    use libimagstore::store::FileLockEntry;

    pub struct HasStartTimeWhere<F>(F)
        where F: Fn(&NaiveDateTime) -> bool;

    impl<F: Fn(&NaiveDateTime) -> bool> HasStartTimeWhere<F> {
        pub fn new(f: F) -> HasStartTimeWhere<F> {
            HasStartTimeWhere(f)
        }
    }

    impl<'a, F> Filter<FileLockEntry<'a>> for HasStartTimeWhere<F>
        where F: Fn(&NaiveDateTime) -> bool
    {
        fn filter(&self, entry: &FileLockEntry) -> bool {
            entry.get_start_datetime()
                .map(|o| o.map(|dt| (self.0)(&dt)).unwrap_or(false))
                .unwrap_or(false)
        }
    }

    pub struct HasEndTimeWhere<F>(F)
        where F: Fn(&NaiveDateTime) -> bool;

    impl<F: Fn(&NaiveDateTime) -> bool> HasEndTimeWhere<F> {
        pub fn new(f: F) -> HasEndTimeWhere<F> {
            HasEndTimeWhere(f)
        }
    }

    impl<'a, F> Filter<FileLockEntry<'a>> for HasEndTimeWhere<F>
        where F: Fn(&NaiveDateTime) -> bool
    {
        fn filter(&self, entry: &FileLockEntry) -> bool {
            entry.get_end_datetime()
                .map(|o| o.map(|dt| (self.0)(&dt)).unwrap_or(false))
                .unwrap_or(false)
        }
    }

    pub struct HasOneOfTags<'a>(&'a Vec<TTT>);

    impl<'a> HasOneOfTags<'a> {
        pub fn new(tags: &'a Vec<TTT>) -> HasOneOfTags<'a> {
            HasOneOfTags(tags)
        }
    }

    impl<'a, 'b> Filter<FileLockEntry<'b>> for HasOneOfTags<'a> {
        fn filter(&self, entry: &FileLockEntry) -> bool {
            entry.get_timetrack_tag().map(|t| self.0.contains(&t)).unwrap_or(false)
        }
    }

}
pub use self::types::HasStartTimeWhere;
pub use self::types::HasEndTimeWhere;
pub use self::types::HasOneOfTags;

