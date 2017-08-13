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

use semver::Version;

use libimagstore::store::Entry;

use builtin::header::version::gt::VersionGt;
use builtin::header::version::lt::VersionLt;
use filters::filter::Filter;
use filters::ops::and::And;
use filters::ops::not::Not;

pub struct VersionInRange {
    and: And<VersionGt, VersionLt>,
}

impl VersionInRange {

    pub fn new(lowerbound: Version, upperbound: Version) -> VersionInRange {
        VersionInRange { and: VersionGt::new(lowerbound).and(VersionLt::new(upperbound)) }
    }

}

impl Filter<Entry> for VersionInRange {

    fn filter(&self, e: &Entry) -> bool {
        self.and.filter(e)
    }

}

pub struct VersionOutOfRange {
    not: Not<VersionInRange>
}

impl VersionOutOfRange {

    pub fn new(lowerbound: Version, upperbound: Version) -> VersionOutOfRange {
        VersionOutOfRange { not: VersionInRange::new(lowerbound, upperbound).not() }
    }

}

impl Filter<Entry> for VersionOutOfRange {

    fn filter(&self, e: &Entry) -> bool {
        self.not.filter(e)
    }

}

