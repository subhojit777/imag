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

use libimagentryutil::isa::Is;
use libimagentryutil::isa::IsKindHeaderPathProvider;

use error::CategoryError as CE;
use store::CATEGORY_REGISTER_NAME_FIELD_PATH;

provide_kindflag_path!(pub IsCategory, "category.is_category");

pub trait Category {
    fn is_category(&self)                   -> Result<bool>;
    fn get_name(&self)                      -> Result<String>;
    fn get_entries(&self, store: &Store)    -> Result<StoreIdIterator>;
}

impl Category for Entry {
    fn is_category(&self) -> Result<bool> {
        self.is::<IsCategory>().map_err(CE::from)
    }

    fn get_name(&self) -> Result<String> {
        self.get_header().read_string(CATEGORY_REGISTER_NAME_FIELD_PATH).map_err(CE::from)
    }

    fn get_entries(&self, store: &Store) -> Result<CategoryIdIterator> {
        unimplemented!()
    }
}

