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

error_chain! {
    types {
        ViewError, ViewErrorKind, ResultExt, Result;
    }

    links {
        StoreError(::libimagstore::error::StoreError, ::libimagstore::error::StoreErrorKind);
    }

    errors {
        Unknown              {
            description("Unknown view error")
            display("Unknown view error")
        }

        GlobError            {
            description("Error while glob()ing")
            display("Error while glob()ing")
        }

        PatternError         {
            description("Error in glob() pattern")
            display("Error in glob() pattern")
        }

        PatternBuildingError {
            description("Could not build glob() pattern")
            display("Could not build glob() pattern")
        }

        ViewError            {
            description("Failed to start viewer")
            display("Failed to start viewer")
        }

    }
}

