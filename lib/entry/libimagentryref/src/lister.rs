//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015, 2016 Matthias Beyer <mail@beyermatthias.de> and contributors
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

use std::default::Default;
use std::io::stdout;
use std::io::Write;
use std::ops::Deref;

use libimagentrylist::lister::Lister;
use libimagentrylist::error::Result;
use libimagerror::trace::trace_error;
use libimagstore::store::FileLockEntry;
use libimagentrylist::error::ListErrorKind as LEK;
use libimagentrylist::error as lerror;

use reference::Ref;

pub struct RefLister {
    check_dead: bool,
    check_changed: bool,
    check_changed_content: bool,
    check_changed_permiss: bool,
}

impl RefLister {

    pub fn new() -> RefLister {
        RefLister::default()
    }

    pub fn check_dead(mut self, b: bool) -> RefLister {
        self.check_dead = b;
        self
    }

    pub fn check_changed(mut self, b: bool) -> RefLister {
        self.check_changed = b;
        self
    }

    pub fn check_changed_content(mut self, b: bool) -> RefLister {
        self.check_changed_content = b;
        self
    }

    pub fn check_changed_permiss(mut self, b: bool) -> RefLister {
        self.check_changed_permiss = b;
        self
    }

}

impl Default for RefLister {

    fn default() -> RefLister {
        RefLister {
            check_dead: false,
            check_changed: false,
            check_changed_content: false,
            check_changed_permiss: false,
        }
    }
}

impl Lister for RefLister {

    fn list<'b, I: Iterator<Item = FileLockEntry<'b>>>(&self, entries: I) -> Result<()> {

        debug!("Called list()");
        let (r, n) = entries.fold((Ok(()), 0), |(accu, i), entry| {
            debug!("fold({:?}, {:?})", accu, entry);
            let r = accu.and_then(|_| {
                    debug!("Listing Entry: {:?}", entry);
                    {
                        let is_dead = if self.check_dead {
                            if try!(lerror::ResultExt::chain_err(entry.fs_link_exists(), || LEK::FormatError)) {
                                "dead"
                            } else {
                                "alive"
                            }
                        } else {
                            "not checked"
                        };

                        let is_changed = if self.check_changed {
                            if check_changed(entry.deref()) { "changed" } else { "unchanged" }
                        } else {
                            "not checked"
                        };

                        let is_changed_content = if self.check_changed_content {
                            if check_changed_content(entry.deref()) { "changed" } else { "unchanged" }
                        } else {
                            "not checked"
                        };

                        let is_changed_permiss = if self.check_changed_permiss {
                            if check_changed_permiss(entry.deref()) { "changed" } else { "unchanged" }
                        } else {
                            "not checked"
                        };

                        Ok(format!("{} | {} | {} | {} | {} | {}",
                                is_dead,
                                is_changed,
                                is_changed_content,
                                is_changed_permiss,
                                entry.get_path_hash().unwrap_or_else(|_| String::from("Cannot get hash")),
                                entry.get_location()))
                    }
                        .and_then(|s| {
                            lerror::ResultExt::chain_err(write!(stdout(), "{}\n", s), || LEK::FormatError)
                        })
                })
                .map(|_| ());
            (r, i + 1)
        });
        debug!("Iterated over {} entries", n);
        r
    }

}

fn check_changed<R: Ref>(r: &R) -> bool {
    check_changed_content(r) && check_changed_permiss(r)
}

fn check_changed_content<R: Ref>(r: &R) -> bool {
    let eq = r.get_current_hash()
        .and_then(|hash| r.get_stored_hash().map(|stored| (hash, stored)))
        .map(|(hash, stored)| hash == stored);

    match eq {
        Ok(eq) => eq,
        Err(e) => {
            warn!("Could not check whether the ref changed on the FS");
            trace_error(&e);

            // We continue here and tell the callee that this reference is unchanged
            false
        },
    }
}

fn check_changed_permiss<R: Ref>(_: &R) -> bool {
    warn!("Permission changes tracking not supported yet.");
    false
}

