use std::io::stdout;
use std::io::Write;

use lister::Lister;
use result::Result;

use libimagstore::store::FileLockEntry;

pub struct PathLister {
    absolute: bool,
}

impl PathLister {

    pub fn new(absolute: bool) -> PathLister {
        PathLister {
            absolute: absolute,
        }
    }

}

impl Lister for PathLister {

    fn list<'a, I: Iterator<Item = FileLockEntry<'a>>>(&self, entries: I) -> Result<()> {
        use error::ListError as LE;
        use error::ListErrorKind as LEK;

        entries.fold(Ok(()), |accu, entry| {
            accu.and_then(|_| Ok(entry.get_location().clone()))
                .and_then(|pb| {
                    if self.absolute {
                        pb.canonicalize().map_err(|e| LE::new(LEK::FormatError, Some(Box::new(e))))
                    } else {
                        Ok(pb)
                    }
                })
                .and_then(|pb| {
                    write!(stdout(), "{:?}\n", pb)
                        .map_err(|e| LE::new(LEK::FormatError, Some(Box::new(e))))
                })
                .map_err(|e| {
                    if e.err_type() == LEK::FormatError {
                        e
                    } else {
                        LE::new(LEK::FormatError, Some(Box::new(e)))
                    }
                })
            })
    }

}

