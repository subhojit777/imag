use std::io::stdout;
use std::io::Write;

use lister::Lister;
use result::Result;

use libimagstore::store::FileLockEntry;
use libimagstore::store::Entry;

pub struct CoreLister<T: Fn(&Entry) -> String> {
    lister: Box<T>,
}

impl<T: Fn(&Entry) -> String> CoreLister<T> {

    pub fn new(lister: T) -> CoreLister<T> {
        CoreLister {
            lister: Box::new(lister),
        }
    }

}

impl<T: Fn(&Entry) -> String> Lister for CoreLister<T> {

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

