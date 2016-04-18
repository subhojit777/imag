use libimagstore::store::Entry;

use result::Result;

pub trait Viewer {

    fn view_entry(&self, e: &Entry) -> Result<()>;

    fn view_entries<I: Iterator<Item = Entry>>(&self, entries: I) -> Result<()> {
        for entry in entries {
            if let Err(e) = self.view_entry(&entry) {
                return Err(e);
            }
        }
        Ok(())
    }
}
