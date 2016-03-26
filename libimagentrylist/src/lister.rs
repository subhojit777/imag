use libimagstore::store::FileLockEntry;

use result::Result;

pub trait Lister {

    fn list<'a, I: Iterator<Item = FileLockEntry<'a>>>(&self, entries: I) -> Result<()>;

}

