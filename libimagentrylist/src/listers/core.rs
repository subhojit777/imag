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

        debug!("Called list()");
        let (r, n) = entries
            .fold((Ok(()), 0), |(accu, i), entry| {
                debug!("fold({:?}, {:?})", accu, entry);
                let r = accu.and_then(|_| {
                        debug!("Listing Entry: {:?}", entry);
                        write!(stdout(), "{:?}\n", (self.lister)(&entry))
                            .map_err(|e| LE::new(LEK::FormatError, Some(Box::new(e))))
                    });
                (r, i + 1)
            });
        debug!("Iterated over {} entries", n);
        r
    }

}

