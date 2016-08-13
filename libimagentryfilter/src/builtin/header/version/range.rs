use semver::Version;

use libimagstore::store::Entry;

use builtin::header::version::gt::VersionGt;
use builtin::header::version::lt::VersionLt;
use filters::filter::Filter;
use ops::and::And;
use ops::not::Not;

pub struct VersionInRange {
    and: And,
}

impl VersionInRange {

    pub fn new(lowerbound: Version, upperbound: Version) -> VersionInRange {
        VersionInRange { and: VersionGt::new(lowerbound).and(Box::new(VersionLt::new(upperbound))) }
    }

}

impl Filter for VersionInRange {

    fn filter(&self, e: &Entry) -> bool {
        self.and.filter(e)
    }

}

pub struct VersionOutOfRange {
    not: Not
}

impl VersionOutOfRange {

    pub fn new(lowerbound: Version, upperbound: Version) -> VersionOutOfRange {
        VersionOutOfRange { not: VersionInRange::new(lowerbound, upperbound).not() }
    }

}

impl Filter for VersionOutOfRange {

    fn filter(&self, e: &Entry) -> bool {
        self.not.filter(e)
    }

}

