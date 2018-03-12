//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015-2018 Matthias Beyer <mail@beyermatthias.de> and contributors
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; version
// 2.1 of the License.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
//

use std::path::Path;
use std::path::PathBuf;
use std::result::Result as RResult;
use std::ffi::OsStr;

use vobject::parse_component;
use uuid::Uuid;

use libimagstore::store::Store;
use libimagstore::store::FileLockEntry;
use libimagstore::storeid::StoreIdIterator;
use libimagentryref::refstore::RefStore;
use libimagentryref::refstore::UniqueRefPathGenerator;
use libimagentryref::generators::sha1::Sha1;
use libimagentryutil::isa::Is;

use contact::IsContact;
use error::ContactError as CE;
use error::Result;
use util;

pub struct UniqueContactPathGenerator;
impl UniqueRefPathGenerator for UniqueContactPathGenerator {
    type Error = CE;

    /// The collection the `StoreId` should be created for
    fn collection() -> &'static str {
        "contact"
    }

    /// A function which should generate a unique string for a Path
    fn unique_hash<A: AsRef<Path>>(path: A) -> RResult<String, Self::Error> {
        debug!("Generating unique hash for path: {:?}", path.as_ref());

        if let Some(p) = path.as_ref().file_stem().and_then(OsStr::to_str).map(String::from) {
            debug!("Found UUID string: '{}'", p);
            Uuid::parse_str(&p)
                .map_err(CE::from)
                .map(|u| format!("{}", u.hyphenated())) // FIXME I don't know how to do in not-ugly
        } else { // else, we sha1 the (complete) content
            debug!("Couldn't find UUID string, using SHA1 of contents");
            Sha1::unique_hash(path).map_err(CE::from)
        }
    }

}

pub trait ContactStore<'a> : RefStore<'a> {

    // creating

    fn create_from_path(&'a self, p: &PathBuf) -> Result<FileLockEntry<'a>>;

    /// Create contact ref from buffer
    ///
    /// Needs the `p` argument as we're finally creating a reference by path, the buffer is only for
    /// collecting metadata.
    fn create_from_buf<P: AsRef<Path>>(&'a self, p: P, buf: &String) -> Result<FileLockEntry<'a>>;

    // getting

    fn all_contacts(&'a self) -> Result<StoreIdIterator>;
}

/// The extension for the Store to work with contacts
///
/// The contact functionality is implemented by using the `libimagentryref` library, so basically
/// we only reference vcard files from outside the store.
///
/// Because of this, we do not have an own store collection `/contacts` or something like that, but
/// must stress the `libimagentryref` API for everything.
impl<'a> ContactStore<'a> for Store {

    fn create_from_path(&'a self, p: &PathBuf) -> Result<FileLockEntry<'a>> {
        util::read_to_string(p).and_then(|buf| self.create_from_buf(p, &buf))
    }

    /// Create contact ref from buffer
    fn create_from_buf<P: AsRef<Path>>(&'a self, p: P, buf: &String) -> Result<FileLockEntry<'a>> {
        let component = parse_component(&buf)?;
        debug!("Parsed: {:?}", component);

        RefStore::create_ref::<UniqueContactPathGenerator, P>(self, p)
            .map_err(From::from)
            .and_then(|mut entry| {
                entry.set_isflag::<IsContact>()
                    .map_err(From::from)
                    .map(|_| entry)
            })
    }

    fn all_contacts(&'a self) -> Result<StoreIdIterator> {
        let iter = self
            .entries()?
            .without_store()
            .filter(|id| id.is_in_collection(&["contact"]));

        Ok(StoreIdIterator::new(Box::new(iter)))
    }

}

