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
use libimagstore::hook::error::HookErrorKind as HEK;
use libimagstore::hook::accessor::HookDataAccessor as HDA;
use libimagstore::hook::accessor::HookDataAccessorProvider;
use libimagstore::hook::accessor::NonMutableHookDataAccessor;
use libimagstore::hook::result::HookResult;
use libimagstore::store::FileLockEntry;
use libimagentrylink::internal::InternalLinker;
use libimagerror::trace::trace_error;


mod error {
    generate_error_imports!();
    generate_error_types!(NoLinksLeftCheckerHookError, NoLinksLeftCheckerHookErrorKind,
        LinksLeft => "The entry has links and therefor cannot be deleted."
    );
}
use self::error::NoLinksLeftCheckerHookError as NLLCHE;
use self::error::NoLinksLeftCheckerHookErrorKind as NLLCHEK;
use self::error::MapErrInto;

#[derive(Debug, Clone)]
pub struct DenyDeletionOfLinkedEntriesHook {
    abort: bool
}

impl DenyDeletionOfLinkedEntriesHook {

    pub fn new() -> DenyDeletionOfLinkedEntriesHook {
        DenyDeletionOfLinkedEntriesHook {
            abort: true // by default, this hook aborts actions
        }
    }

}

impl Hook for DenyDeletionOfLinkedEntriesHook {

    fn name(&self) -> &'static str {
        "stdhook_linked_entries_cannot_be_deleted"
    }

    fn set_config(&mut self, v: &Value) {
        self.abort = match v.lookup("aborting") {
            Some(&Value::Boolean(b)) => b,
            Some(_) => {
                warn!("Configuration error, 'aborting' must be a Boolean (true|false).");
                warn!("Assuming 'true' now.");
                true
            },
            None => {
                warn!("No key 'aborting' - Assuming 'true'");
                true
            },
        };
    }

}

impl HookDataAccessorProvider for DenyDeletionOfLinkedEntriesHook {

    fn accessor(&self) -> HDA {
        HDA::NonMutableAccess(self)
    }

}

impl NonMutableHookDataAccessor for DenyDeletionOfLinkedEntriesHook {

    fn access(&self, fle: &FileLockEntry) -> HookResult<()> {
        use libimagutil::warn_result::*;
        use libimagutil::debug_result::*;
        use libimagerror::trace::MapErrTrace;
        use libimagerror::into::IntoError;
        use libimagstore::hook::error::MapErrInto;

        debug!("[NO LINKS LEFT CHECKER HOOK] {:?}", fle.get_location());

        let n = fle
            .get_internal_links()
            .map(|i| i.count())
            .map_warn_err_str("[NO LINKS LEFT CHECKER HOOK]: Cannot get internal links")
            .map_warn_err_str("[NO LINKS LEFT CHECKER HOOK]: Assuming 1 to automatically abort")
            .map_dbg_err_str("[NO LINKS LEFT CHECKER HOOK]: Printing trace now")
            .map_err_trace()
            .unwrap_or(1);

        if n > 0 {
            Err(NLLCHEK::LinksLeft.into_error())
                .map_err(Box::new)
                .map_err(|e| HEK::HookExecutionError.into_error_with_cause(e))
        } else {
            Ok(())
        }
    }

}


