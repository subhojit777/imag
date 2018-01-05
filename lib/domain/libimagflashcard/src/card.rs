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

use toml_query::read::TomlValueReadExt;
use toml::Value;

use libimagstore::store::Entry;
use libimagstore::storeid::StoreId;
use libimagstore::storeid::StoreIdIterator;
use libimagentryutil::isa::Is;
use libimagentryutil::isa::IsKindHeaderPathProvider;

provide_kindflag_path!(pub IsCard, "flashcard.is_card");

use error::Result;
use error::FlashcardError as FE;
use error::FlashcardErrorKind as FEK;

pub trait Card {

    fn is_card(&self)    -> Result<bool>;
    fn question(&self)   -> Result<String>;
    fn answers(&self)    -> Result<Vec<String>>;

}

impl Card for Entry {

    fn is_card(&self)    -> Result<bool> {
        self.is::<IsCard>().map_err(From::from)
    }

    fn question(&self)   -> Result<String> {
        let field = "flashcard.card.question";

        match self.get_header().read(field).map_err(FE::from)? {
            Some(&Value::String(ref s)) => Ok(s.clone()),
            None                        => Err(FEK::HeaderFieldMissing(field)),
            Some(_)                     => Err(FEK::HeaderTypeError("String")),
        }.map_err(FE::from)
    }

    fn answers(&self)    -> Result<Vec<String>> {
        let field = "flashcard.card.answers";

        match self.get_header().read(field).map_err(FE::from)? {
            Some(&Value::Array(ref a)) => {
                let mut res = vec![];
                for elem in a {
                    match *elem {
                        Value::String(ref s) => res.push(s.clone()),
                        _  => return Err(FEK::HeaderTypeError("Array<String>").into()),
                    }
                }
                Ok(res)
            },
            None    => Err(FEK::HeaderFieldMissing(field)),
            Some(_) => Err(FEK::HeaderTypeError("String")),
        }.map_err(FE::from)
    }

}
