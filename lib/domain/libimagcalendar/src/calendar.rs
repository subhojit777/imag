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

use std::path::Path;

use error::Result;
use error::CalendarError as CE;
use error::CalendarErrorKind as CEK;
use event::IsEvent;

use libimagstore::storeid::IntoStoreId;
use libimagstore::store::Store;
use libimagstore::store::Entry;
use libimagstore::store::FileLockEntry;
use libimagentryutil::isa::Is;
use libimagentryutil::isa::IsKindHeaderPathProvider;
use libimagentryref::reference::Ref;

use toml::Value;
use toml_query::insert::TomlValueInsertExt;
use vobject::icalendar::ICalendar;

provide_kindflag_path!(pub IsCalendar, "calendar.is_calendar");

/// A Calendar is a set of calendar entries
///
/// A Calendar represents a ical file on the filesystem
pub trait Calendar : Ref {
    fn is_calendar(&self) -> Result<bool>;

    fn calendar(&self) -> Result<ICalendar>;
    fn events<'a>(&mut self, store: &'a Store) -> Result<Vec<FileLockEntry<'a>>>;
}

impl Calendar for Entry {
    fn is_calendar(&self) -> Result<bool> {
        self.is::<IsCalendar>().map_err(CE::from)
    }

    fn calendar(&self) -> Result<ICalendar> {
        self.get_path()
            .map_err(CE::from)
            .and_then(::util::readfile)
            .and_then(|s| ICalendar::build(&s).map_err(CE::from))
    }

    /// Build the events for the calendar
    ///
    /// This function builds "Event" objects at "calendar/event/{event uid}" which refers to the
    /// calendar file (the same which is refered to from this object).
    ///
    /// If the event object already exists, it is loaded from the `store`.
    ///
    /// Events are automatically linked to the calendar
    ///
    /// # Warning
    ///
    /// If an event does not have an "UID", an error will be generated for that file.
    /// This may change in future.
    ///
    fn events<'a>(&mut self, store: &'a Store) -> Result<Vec<FileLockEntry<'a>>> {
        use module_path::ModuleEntryPath;

        self.calendar()
            .and_then(|c| {
                c.events()
                    .map(|event_r| {
                        let path = self.get_path()?;

                        event_r
                            .map_err(|_| CEK::NotAnEvent(path.clone()).into())
                            .and_then(|e| {
                                e.get_uid()
                                    .ok_or_else(|| CEK::EventWithoutUid(path.clone()).into())
                            })
                            .and_then(|event_id| {
                                let s = format!("event/{}", event_id.raw());
                                ModuleEntryPath::new(s)
                                    .into_storeid()
                                    .map_err(CE::from)
                                    .map(|sid| (event_id, sid))
                            })
                            .and_then(|(eid, sid)| Ok((eid, store.retrieve(sid)?)))
                            .and_then(move |(eid, mut fle)| {
                                let _ = fle.make_ref(eid.raw().clone(), path)?;
                                let _ = fle.set_isflag::<IsEvent>()?;
                                let _ = fle.get_header_mut()
                                    .insert("calendar.event.uid", Value::String(eid.raw().clone()))?;
                                Ok(fle)
                            })
                    })
                    .collect::<Result<Vec<FileLockEntry>>>()
            })
    }

}

