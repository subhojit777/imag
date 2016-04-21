use std::io::stdout;
use std::io::Write;

use lister::Lister;
use result::Result;

use libimagstore::store::FileLockEntry;
use libimagstore::store::Entry;

pub struct CoreLister<'a> {
    lister: &'a Fn(&Entry) -> String,
}

impl<'a> CoreLister<'a> {

    pub fn new(lister: &'a Fn(&Entry) -> String) -> CoreLister<'a> {
        CoreLister {
            lister: lister,
        }
    }

}

impl<'a> Lister for CoreLister<'a> {

    fn list<'b, I: Iterator<Item = FileLockEntry<'b>>>(&self, entries: I) -> Result<()> {
        use error::ListError as LE;
        use error::ListErrorKind as LEK;

        entries.fold(Ok(()), |accu, entry| {
            accu.and_then(|_| {
                    write!(stdout(), "{:?}\n", (self.lister)(&entry))
                        .map_err(|e| LE::new(LEK::FormatError, Some(Box::new(e))))
                })
            })
    }

}

