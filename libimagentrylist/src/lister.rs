use clap::ArgMatches;

use libimagstore::store::FileLockEntry;

use result::Result;

pub trait Lister : Sized {

    fn list<'a, I: Iterator<Item = FileLockEntry<'a>>>(&self, entries: I) -> Result<()>;

}

