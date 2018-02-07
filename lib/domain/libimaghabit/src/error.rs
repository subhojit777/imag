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
        HabitError, HabitErrorKind, ResultExt, Result;
    }

    links {
        StoreError(::libimagstore::error::StoreError, ::libimagstore::error::StoreErrorKind);
        LinkError(::libimagentrylink::error::LinkError, ::libimagentrylink::error::LinkErrorKind);
        KairosError(::kairos::error::KairosError, ::kairos::error::KairosErrorKind);
        EntryUtilError(::libimagentryutil::error::EntryUtilError, ::libimagentryutil::error::EntryUtilErrorKind);
    }

    foreign_links {
        TomlError(::toml_query::error::Error);
        ChronoError(::chrono::format::ParseError);
    }

    errors {
        HabitBuilderMissing(variable_name: &'static str) {
            description("Habit builder has not all required information")
            display("Habit builder misses {}", variable_name)
        }

        HabitBuilderLogicError(text: &'static str) {
            description("Logic error in Habit builder")
            display("Logic error: {}", text)
        }

        HeaderFieldMissing(path: &'static str) {
            description("Header field missing")
            display("Header field missing: {}", path)
        }

        HeaderTypeError(path: &'static str, required_type: &'static str) {
            description("Header type error")
            display("Header type error: Expected {} at {}, found other type", required_type, path)
        }

    }
}

