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

use error::EntryUtilError as EUE;
use error::Result;

use toml::Value;
use toml_query::read::TomlValueReadTypeExt;
use toml_query::insert::TomlValueInsertExt;

/// Trait to check whether an entry is a certain kind of entry
///
/// If an entry is marked with a `bool` flag in the header to contain a certain amount of data (for
/// example a "wiki" entry may provide some meta information in the `[wiki]` section of its header),
/// this trait provides a check whether the entry has set the flag to `true` or `false`.
///
/// # Note
///
/// This trait is solely for library implementations, as convenience functionality for implementing
/// some function like this:
///
/// # Example
///
/// ```
/// # extern crate libimagstore;
/// # #[macro_use]
/// # extern crate libimagentryutil;
///
/// # use libimagentryutil::error::Result as Result;
/// use libimagentryutil::isa::IsKindHeaderPathProvider;
/// use libimagentryutil::isa::Is;
///
/// trait WikiArticle {
///     fn is_wiki_article(&self) -> Result<bool>;
///     // ...
/// }
///
/// provide_kindflag_path!(IsWikiEntry, "wiki.is_entry");
///
/// impl WikiArticle for ::libimagstore::store::Entry {
///     fn is_wiki_article(&self) -> Result<bool> {
///         self.is::<IsWikiEntry>()
///     }
/// }
///
/// # fn main() { }
/// ```
///
/// # See also
///
/// * Documentation for `IsKindHeaderPathProvider`
/// * Helper macro `provide_kindflag_path!()`
///
pub trait Is {
    fn is<T: IsKindHeaderPathProvider>(&self) -> Result<bool>;
    fn set_isflag<T: IsKindHeaderPathProvider>(&mut self) -> Result<()>;
}

impl Is for ::libimagstore::store::Entry {
    fn is<T: IsKindHeaderPathProvider>(&self) -> Result<bool> {
        let field = T::kindflag_header_location();

        match self.get_header().read_bool(field)? {
            Some(b) => Ok(b),
            None    => Err(format!("Field {} not available", field)).map_err(EUE::from),
        }
    }

    fn set_isflag<T: IsKindHeaderPathProvider>(&mut self) -> Result<()> {
        self.get_header_mut()
            .insert(T::kindflag_header_location(), Value::Boolean(true))
            .map_err(EUE::from)
            .map(|_| ())
    }
}


/// The IsKindHeaderPathProvider trait provides a function `typeflag_header_location()` which
/// returns a `toml-query` path.
///
/// This path points to a `bool` entry in the header of an entry which marks the entry to be an
/// entry of a certain kind.
///
/// For example, an "Wiki" entry might contain a `true` at `"wiki.is_entry"` in the header.
/// This trait provides `"wiki.is_entry"`.
pub trait IsKindHeaderPathProvider {
    fn kindflag_header_location() -> &'static str;
}

/// Create (pub/non-pub) type for providing a `kindflag_header_location()` implementation.
#[macro_export]
macro_rules! provide_kindflag_path {
    (pub $entry_header_path_provider_type:ident, $path:expr) => {
        pub struct $entry_header_path_provider_type;
        provide_kindflag_path!(impl for $entry_header_path_provider_type, $path);
    };

    ($entry_header_path_provider_type:ident, $path:expr) => {
        struct $entry_header_path_provider_type;
        provide_kindflag_path!(impl for $entry_header_path_provider_type, $path);
    };

    (impl for $entry_header_path_provider_type:ident, $path:expr) => {
        impl IsKindHeaderPathProvider for $entry_header_path_provider_type {
            fn kindflag_header_location() -> &'static str {
                $path
            }
        }
    };
}

