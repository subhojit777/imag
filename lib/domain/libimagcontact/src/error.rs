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

use libimagstore::storeid::StoreId;

error_chain! {
    types {
        ContactError, ContactErrorKind, ResultExt, Result;
    }

    links {
        StoreError(::libimagstore::error::StoreError, ::libimagstore::error::StoreErrorKind);
        RefError(::libimagentryref::error::RefError, ::libimagentryref::error::RefErrorKind);
        VObjectError(::vobject::error::VObjectError, ::vobject::error::VObjectErrorKind);
        EntryUtilError(::libimagentryutil::error::EntryUtilError, ::libimagentryutil::error::EntryUtilErrorKind);
    }

    foreign_links {
        Io(::std::io::Error);
        TomlQueryError(::toml_query::error::Error);
        UuidError(::uuid::ParseError);
    }

    errors {

        HeaderTypeError(ty: &'static str, loc: &'static str) {
            description("Type error in header")
            display("Type error in header, expected {} at '{}', found other type", ty, loc)
        }

        EntryNotFound(sid: StoreId) {
            description("Entry not found with StoreId")
            display("Entry {:?} not found", sid)
        }

        UidMissing(path: String) {
            description("Vcard object has no UID")
            display("Vcard at {:?} has no UID", path)
        }

    }
}

