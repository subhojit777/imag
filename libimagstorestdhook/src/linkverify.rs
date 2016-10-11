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

use std::path::PathBuf;

use toml::Value;

use libimagstore::hook::Hook;
use libimagstore::hook::accessor::HookDataAccessor as HDA;
use libimagstore::hook::accessor::HookDataAccessorProvider;
use libimagstore::hook::accessor::NonMutableHookDataAccessor;
use libimagstore::hook::result::HookResult;
use libimagstore::store::FileLockEntry;
use libimagentrylink::internal::InternalLinker;
use libimagerror::trace::trace_error;

#[derive(Debug, Clone)]
pub struct LinkedEntriesExistHook {
    store_location: PathBuf,
}

impl LinkedEntriesExistHook {

    pub fn new(store_location: PathBuf) -> LinkedEntriesExistHook {
        LinkedEntriesExistHook {
            store_location: store_location,
        }
    }

}

impl Hook for LinkedEntriesExistHook {

    fn name(&self) -> &'static str {
        "stdhook_linked_entries_exist"
    }

    fn set_config(&mut self, _: &Value) {
        () // We are not configurable here.
    }

}

impl HookDataAccessorProvider for LinkedEntriesExistHook {

    fn accessor(&self) -> HDA {
        HDA::NonMutableAccess(self)
    }

}

impl NonMutableHookDataAccessor for LinkedEntriesExistHook {

    fn access(&self, fle: &FileLockEntry) -> HookResult<()> {
        debug!("[LINKVERIFY HOOK] {:?}", fle.get_location());
        let _ = fle.get_internal_links()
            .map(|links| {
                for link in links {
                    if !link.exists() {
                        warn!("File link does not exist: {:?} -> {:?}", fle.get_location(), link);
                    }
                }
            })
            .map_err(|e| {
                warn!("Couldn't execute Link-Verify hook");
                trace_error(&e);
            });
        Ok(())
    }

}

