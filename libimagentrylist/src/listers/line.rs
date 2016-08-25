use std::io::stdout;
use std::io::Write;

use lister::Lister;
use result::Result;

use libimagstore::store::FileLockEntry;
use libimagutil::iter::FoldResult;

pub struct LineLister<'a> {
    unknown_output: &'a str,
}

impl<'a> LineLister<'a> {

    pub fn new(unknown_output: &'a str) -> LineLister<'a> {
        LineLister {
            unknown_output: unknown_output,
        }
    }

}

impl<'a> Lister for LineLister<'a> {

    fn list<'b, I: Iterator<Item = FileLockEntry<'b>>>(&self, entries: I) -> Result<()> {
        use error::ListError as LE;
        use error::ListErrorKind as LEK;

        entries.fold_defresult(|entry| {
            let s = entry.get_location().to_str().unwrap_or(String::from(self.unknown_output));
            write!(stdout(), "{:?}\n", s).map_err(|e| LE::new(LEK::FormatError, Some(Box::new(e))))
        })
    }

}
