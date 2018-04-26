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

pub mod create;
pub mod filter;
pub mod get;
pub mod setendtime;
pub mod storeid;
pub mod tag;

#[cfg(test)]
mod test {
    use chrono::naive::NaiveDate;

    use libimagstore::store::Store;

    use super::setendtime::*;
    use super::tag::*;

    fn get_store() -> Store {
        use std::path::PathBuf;
        use libimagstore::file_abstraction::InMemoryFileAbstraction;

        let backend = Box::new(InMemoryFileAbstraction::default());
        Store::new_with_backend(PathBuf::from("/"), &None, backend).unwrap()
    }

    #[test]
    fn test_building_chain() {
        let now   = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 1);
        let then  = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 2);
        let store = get_store();
        let tags  = vec!["foo", "bar"];

        let iter = tags.into_iter().map(String::from);

        let _ : SetEndTimeIter = TagIter::new(Box::new(iter))
            .create_storeids(now)
            .create_entries(&store)
            .set_end_time(then);
        // just to see whether this compiles, actually.
    }
}

