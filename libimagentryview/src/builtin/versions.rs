use libimagstore::store::Entry;
use libimagstore::store::Store;
use libimagerror::into::IntoError;

use viewer::Viewer;
use result::Result;
use error::ViewErrorKind as VEK;

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
            .storified(self.store)
            .to_str()
            .and_then(|s| s.split("~").next())
            .map(|component| {
                glob(&format!("{}~*", component)[..])
                    .map_err(|_| VEK::PatternError.into_error())
                    .and_then(|paths| {
                        for entry in paths {
                            let p = match entry {
                                Err(_) => return Err(VEK::GlobError.into_error()),
                                Ok(p) => p,
                            };
                            let p = p.file_name()
                                .and_then(|s| s.to_str())
                                .unwrap(); // TODO
                            println!("{}", p);
                        };
                        Ok(())
                    })
            })
            .unwrap_or(Err(VEK::PatternBuildingError.into_error()))
    }

}

