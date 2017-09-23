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

use libimagstore::storeid::StoreId;

error_chain! {
    types {
        LinkError, LinkErrorKind, ResultExt, Result;
    }

    links {
        StoreError(::libimagstore::error::StoreError, ::libimagstore::error::StoreErrorKind);
    }

    foreign_links {
        TomlQueryError(::toml_query::error::Error);
    }

    errors {
        EntryHeaderReadError    {
            description("Error while reading an entry header")
            display("Error while reading an entry header")
        }

        EntryHeaderWriteError   {
            description("Error while writing an entry header")
            display("Error while writing an entry header")
        }

        ExistingLinkTypeWrong   {
            description("Existing link entry has wrong type")
            display("Existing link entry has wrong type")
        }

        LinkTargetDoesNotExist  {
            description("Link target does not exist in the store")
            display("Link target does not exist in the store")
        }

        LinkParserError         {
            description("Link cannot be parsed")
            display("Link cannot be parsed")
        }

        LinkParserFieldMissingError {
            description("Link cannot be parsed: Field missing")
            display("Link cannot be parsed: Field missing")
        }

        LinkParserFieldTypeError {
            description("Link cannot be parsed: Field type wrong")
            display("Link cannot be parsed: Field type wrong")
        }

        InternalConversionError {
            description("Error while converting values internally")
            display("Error while converting values internally")
        }

        InvalidUri              {
            description("URI is not valid")
            display("URI is not valid")
        }

        DeadLink(from: StoreId, to: StoreId) {
            description("Dead link")
            display("Dead link from: {from} to: {to}", from = from, to = to)
        }

        LinkHandlingError {
            description("Error in link handling")
            display("Error in link handling")
        }
    }
}

