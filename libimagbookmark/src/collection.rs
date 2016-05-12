//! BookmarkCollection module
//!
//! A BookmarkCollection is nothing more than a simple store entry. One can simply call functions
//! from the libimagentrylink::external::ExternalLinker trait on this to generate external links.
//!
//! The BookmarkCollection type offers helper functions to get all links or such things.
use std::ops::Deref;
use std::ops::DerefMut;

use error::BookmarkError as BE;
use error::BookmarkErrorKind as BEK;
use error::MapErrInto;
use result::Result;
use module_path::ModuleEntryPath;

use libimagstore::store::Store;
use libimagstore::storeid::IntoStoreId;
use libimagstore::store::FileLockEntry;
use libimagentrylink::external::ExternalLinker;
use libimagentrylink::internal::InternalLinker;
use libimagentrylink::internal::Link;
use url::Url;

pub struct BookmarkCollection<'a> {
    fle: FileLockEntry<'a>,
    store: &'a Store,
}

/// {Internal, External}Linker is implemented as Deref is implemented
impl<'a> Deref for BookmarkCollection<'a> {
    type Target = FileLockEntry<'a>;

    fn deref(&self) -> &FileLockEntry<'a> {
        &self.fle
    }

}

impl<'a> DerefMut for BookmarkCollection<'a> {

    fn deref_mut(&mut self) -> &mut FileLockEntry<'a> {
        &mut self.fle
    }

}

impl<'a> BookmarkCollection<'a> {

    pub fn new(store: &'a Store, name: &str) -> Result<BookmarkCollection<'a>> {
        let id = ModuleEntryPath::new(name).into_storeid();
        store.create(id)
            .map(|fle| {
                BookmarkCollection {
                    fle: fle,
                    store: store,
                }
            })
            .map_err_into(BEK::StoreReadError)
    }

    pub fn open(store: &Store, name: &str) -> Result<BookmarkCollection<'a>> {
        unimplemented!()
    }

    pub fn delete(store: &Store, name: &str) -> Result<()> {
        unimplemented!()
    }

    pub fn links(&self) -> Result<Vec<Url>> {
        self.fle.get_external_links(&self.store).map_err_into(BEK::LinkError)
    }

    pub fn link_entries(&self) -> Result<Vec<Link>> {
        use libimagentrylink::external::is_external_link_storeid;

        self.fle
            .get_internal_links()
            .map(|v| v.into_iter().filter(|id| is_external_link_storeid(id)).collect())
            .map_err_into(BEK::StoreReadError)
    }

}

