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

use libimagstore::storeid::StoreIdIterator;
use libimagstore::store::FileLockEntry;
use libimagstore::store::Store;
use libimagstore::store::Entry;
use libimagstore::storeid::StoreId;
use libimagentryutil::isa::Is;
use libimagentryutil::isa::IsKindHeaderPathProvider;
use libimagutil::date::datetime_to_string;
use libimagutil::date::datetime_from_string;
use libimagentrylink::internal::InternalLinker;

provide_kindflag_path!(pub IsSession, "flashcard.is_session");

use chrono::NaiveDateTime;
use toml::Value;
use toml_query::insert::TomlValueInsertExt;
use toml_query::read::TomlValueReadExt;

use error::Result;
use error::FlashcardErrorKind as FCEK;
use card::Card;

pub trait Session {
    fn is_session(&self) -> Result<bool>;

    fn start_at(&mut self, ndt: &NaiveDateTime) -> Result<()>;
    fn end_at(&mut self, ndt: &NaiveDateTime)   -> Result<()>;

    fn start(&mut self)                         -> Result<()> {
        let now = ::chrono::offset::Local::now().naive_local();
        self.start_at(&now)
    }

    fn end(&mut self)                           -> Result<()> {
        let now = ::chrono::offset::Local::now().naive_local();
        self.end_at(&now)
    }

    fn started_at(&self) -> Result<Option<NaiveDateTime>>;
    fn ended_at(&self)   -> Result<Option<NaiveDateTime>>;

    fn answer<'a>(&mut self, card: &mut FileLockEntry<'a>, answer: &str) -> Result<bool>;

    /// Get the group this session was created for.
    fn group<'a>(&self, store: &'a Store) -> Result<FileLockEntry<'a>>;
}

impl Session for Entry {
    fn is_session(&self) -> Result<bool> {
        self.is::<IsSession>().map_err(From::from)
    }

    fn start_at(&mut self, ndt: &NaiveDateTime) -> Result<()> {
        self.get_header_mut()
            .insert("flashcard.session.start", Value::String(datetime_to_string(ndt)))
            .map(|_| ())
            .map_err(From::from)
    }

    fn end_at(&mut self, ndt: &NaiveDateTime) -> Result<()> {
        self.get_header_mut()
            .insert("flashcard.session.end", Value::String(datetime_to_string(ndt)))
            .map(|_| ())
            .map_err(From::from)
    }

    fn started_at(&self) -> Result<Option<NaiveDateTime>> {
        match self.get_header().read("flashcard.session.start")? {
            Some(&Value::String(ref s)) => datetime_from_string(s).map(Some).map_err(From::from),
            Some(_) => Err(FCEK::HeaderTypeError("string").into()),
            None    => Err(FCEK::HeaderFieldMissing("flashcard.session.start").into())
        }
    }

    fn ended_at(&self) -> Result<Option<NaiveDateTime>> {
        match self.get_header().read("flashcard.session.end")? {
            Some(&Value::String(ref s)) => datetime_from_string(s).map(Some).map_err(From::from),
            Some(_) => Err(FCEK::HeaderTypeError("string").into()),
            None    => Err(FCEK::HeaderFieldMissing("flashcard.session.end").into())
        }
    }

    fn answer<'a>(&mut self, card: &mut FileLockEntry<'a>, answer: &str) -> Result<bool> {
        let question          = card.question()?;
        let is_correct        = card.answers()?.iter().any(|valid| valid == answer);

        debug!("Answer '{}' for question '{}' is correct = {}", answer, question, is_correct);

        let _                 = self.add_internal_link(card)?;
        let correct_path_elem = if is_correct { "succeeded" } else { "failed" };
        let storeid           = card.get_location().clone().without_base().to_str()?;
        let header_path       = format!("flashcard.session.{}.{}", storeid, correct_path_elem);

        trace!("Reading header at '{}'", header_path);

        match self.get_header_mut().read_mut(&header_path)? {
            Some(&mut Value::Integer(ref mut i)) => {
                trace!("Inserting +1 for existing table for '{}'", storeid);
                *i += 1;
                return Ok(is_correct);
            },
            Some(_) => return Err(unimplemented!()),
            None => {
                // going on...
            }
        }

        {
            trace!("Creating new table for '{}'", storeid);
            let mut init_tab = BTreeMap::new();

            init_tab.insert("succeeded".to_string(), Value::Integer(if is_correct { 1 } else { 0 }));
            init_tab.insert("failed".to_string(),    Value::Integer(if is_correct { 0 } else { 1 }));

            self.get_header_mut().insert(&header_path, Value::Table(init_tab));
        }

        Ok(is_correct)
    }
}

