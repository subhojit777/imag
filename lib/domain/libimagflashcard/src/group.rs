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

use libimagstore::store::Store;
use libimagstore::store::Entry;
use libimagstore::store::FileLockEntry;
use libimagstore::storeid::IntoStoreId;
use libimagstore::storeid::StoreId;
use libimagstore::storeid::StoreIdIterator;
use libimagentryutil::isa::Is;
use libimagentryutil::isa::IsKindHeaderPathProvider;
use libimagentrylink::internal::InternalLinker;

use toml::Value;
use toml_query::read::TomlValueReadExt;
use toml_query::insert::TomlValueInsertExt;

use error::Result;
use error::FlashcardErrorKind as FCEK;
use iter::CardsInGroup;
use iter::SessionsForGroup;
use card::IsCard;
use pathes::mk_card_path;
use pathes::mk_session_path;

provide_kindflag_path!(pub IsCardGroup, "flashcard.is_group");

pub trait CardGroup {
    // Based on libimagentrylink

    fn is_cardgroup(&self) -> Result<bool>;
    fn group_name(&self) -> Result<String>;

    fn create_card<'a>(&mut self, store: &'a Store, question: String, answers: Vec<String>)
        -> Result<FileLockEntry<'a>>;

    fn get_cards<'a>(&self, store: &Store) -> Result<CardsInGroup>;

    fn make_session<'a>(&mut self, store: &'a Store) -> Result<FileLockEntry<'a>>;

    fn sessions<'a>(&mut self, store: &'a Store) -> Result<SessionsForGroup>;

    // TODO: Some stat-functions for the group
    // like percent learned
    // no of cards
    // no of learned cards
    // etc

}

impl CardGroup for Entry {
    fn is_cardgroup(&self) -> Result<bool> {
        self.is::<IsCardGroup>().map_err(From::from)
    }

    fn group_name(&self) -> Result<String> {
        match self.get_header().read("flashcard.group.name")? {
            Some(&Value::String(ref s)) => Ok(s.clone()),
            Some(_)                     => Err(FCEK::HeaderTypeError("string")),
            None                        => Err(FCEK::HeaderFieldMissing("flashcard.group.name")),
        }.map_err(Into::into)
    }

    fn create_card<'a>(&mut self, store: &'a Store, question: String, answers: Vec<String>)
        -> Result<FileLockEntry<'a>>
    {
        let name     = self.group_name()?;
        let cardpath = mk_card_path(&name, &question)?;
        let id       = ::module_path::ModuleEntryPath::new(cardpath).into_storeid()?;
        let mut card = store.create(id)?;

        card.set_isflag::<IsCard>()?;
        {
            let hdr     = card.get_header_mut();
            let answers = answers.into_iter().map(Value::String).collect();

            let _ = hdr.insert("flashcard.card.question", Value::String(question))?;
            let _ = hdr.insert("flashcard.card.answers", Value::Array(answers))?;
        }

        let _ = self.add_internal_link(&mut card)?;
        Ok(card)
    }

    fn get_cards<'a>(&self, store: &Store) -> Result<CardsInGroup> {
        Ok(CardsInGroup::new(store.entries()?.without_store(), self.group_name()?))
    }

    fn make_session<'a>(&mut self, store: &'a Store) -> Result<FileLockEntry<'a>> {
        use session::Session;
        use session::IsSession;
        use libimagutil::date::datetime_to_string;
        use module_path::ModuleEntryPath;

        let gname   = CardGroup::group_name(self)?;
        let now     = ::chrono::offset::Local::now().naive_local();
        let id      = mk_session_path(&gname, &now);
        let id      = ModuleEntryPath::new(id).into_storeid()?;
        let mut fle = store.create(id)?;
        let _ = fle.set_isflag::<IsSession>()?;
        let _ = fle.start_at(&now)?;
        let _ = fle.get_header_mut().insert("flashcard.group.name", Value::String(gname))?;
        let _ = self.add_internal_link(&mut fle)?;
        Ok(fle)
    }

    fn sessions<'a>(&mut self, store: &'a Store) -> Result<SessionsForGroup> {
        Ok(SessionsForGroup::new(store.entries()?.without_store(), self.group_name()?))
    }

}

