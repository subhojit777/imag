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

use libimagrt::runtime::Runtime;
use libimagerror::trace::MapErrTrace;
use libimagstore::storeid::StoreId;
use libimagutil::warn_exit::warn_exit;
use libimagutil::warn_result::*;

pub fn delete(rt: &Runtime) {
    rt.cli()
        .subcommand_matches("delete")
        .map(|sub| {
            sub.value_of("id")
                .map(|id| {
                    let path = PathBuf::from(id);
                    let path = try!(StoreId::new(Some(rt.store().path().clone()), path)
                                    .map_err_trace_exit(1));
                    debug!("Deleting file at {:?}", id);

                    rt.store()
                        .delete(path)
                        .map_warn_err(|e| format!("Error: {:?}", e))
                        .map_err_trace_exit(1)
                })
                .or_else(|| warn_exit("No ID passed. Will exit now", 1))
        })
        .or_else(|| warn_exit("No subcommand 'delete'. Will exit now", 1));
}

#[cfg(test)]
mod tests {
    use create::create;
    use super::delete;

    use std::path::PathBuf;

    make_mock_app! {
        app "imag-link";
        modulename mock;
        version "0.4.0";
        with help "imag-link mocking app";
    }
    use self::mock::generate_test_runtime;
    use self::mock::reset_test_runtime;

    #[test]
    fn test_create_simple() {
        let test_name = "test_create_simple";
        let rt = generate_test_runtime(vec!["create", "-p", "test_create_simple"]).unwrap();

        create(&rt);

        let rt = reset_test_runtime(vec!["delete", "--id", "test_create_simple"], rt).unwrap();

        delete(&rt);

        let e = rt.store().get(PathBuf::from(test_name));
        assert!(e.is_ok());
        let e = e.unwrap();
        assert!(e.is_none());
    }

}

