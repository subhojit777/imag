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

use std::path::PathBuf;

use libimagrt::runtime::Runtime;
use libimagerror::trace::MapErrTrace;
use libimagerror::trace::trace_error_exit;
use libimagstore::storeid::StoreId;
use libimagutil::warn_result::*;

pub fn delete(rt: &Runtime) {
    let scmd  = rt.cli().subcommand_matches("delete").unwrap();
    let id    = scmd.value_of("id").unwrap(); // safe by clap
    let path  = PathBuf::from(id);
    let store = Some(rt.store().path().clone());
    let path  = StoreId::new(store, path).unwrap_or_else(|e| trace_error_exit(&e, 1));
    debug!("Deleting file at {:?}", id);

    let _ = rt.store()
        .delete(path)
        .map_warn_err(|e| format!("Error: {:?}", e))
        .map_err_trace_exit_unwrap(1);
}

#[cfg(test)]
mod tests {
    use create::create;
    use super::delete;

    use std::path::PathBuf;

    make_mock_app! {
        app "imag-store";
        modulename mock;
        version "0.6.3";
        with help "imag-store mocking app";
    }
    use self::mock::generate_test_runtime;
    use self::mock::reset_test_runtime;

    #[test]
    fn test_delete_simple() {
        let test_name = "test_create_simple";
        let rt = generate_test_runtime(vec!["create", "test_create_simple"]).unwrap();

        create(&rt);

        let rt = reset_test_runtime(vec!["delete", "test_create_simple"], rt).unwrap();

        delete(&rt);

        let e = rt.store().get(PathBuf::from(test_name));
        assert!(e.is_ok());
        let e = e.unwrap();
        assert!(e.is_none());
    }

}

