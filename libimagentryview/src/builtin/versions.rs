use libimagstore::store::Entry;
use libimagstore::store::Store;
use libimagerror::into::IntoError;

use viewer::Viewer;
use result::Result;
use error::ViewErrorKind as VEK;
use error::MapErrInto;

pub struct VersionsViewer<'a> {
    store: &'a Store,
}

impl<'a> VersionsViewer<'a> {

    pub fn new(store: &'a Store) -> VersionsViewer<'a> {
        VersionsViewer {
            store: store,
        }
    }

}

impl<'a> Viewer for VersionsViewer<'a> {

    fn view_entry(&self, entr: &Entry) -> Result<()> {
        use glob::glob;

        entr.get_location()
            .clone()
            .with_base(self.store.path().clone())
            .to_str()
            .map_err_into(VEK::ViewError)
            .and_then(|s| {
                s.split("~")
                    .next()
                    .ok_or(VEK::PatternError.into_error())
                    .map(|s| format!("{}~*", s))
                    .and_then(|pat| glob(&pat[..]).map_err(|_| VEK::PatternError.into_error()))
                    .and_then(|paths| {
                        for entry in paths {
                            println!("{}",
                                try!(entry
                                     .map_err(|_| VEK::GlobError.into_error()))
                                     .file_name()
                                     .and_then(|s| s.to_str())
                                     .unwrap() // TODO
                                );
                        };
                        Ok(())
                    })
            })
    }

}

