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

