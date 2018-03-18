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

use chrono::NaiveDateTime;

use toml::Value;
use toml_query::read::TomlValueReadExt;
use vobject::icalendar::ICalendar;
use vobject::icalendar::Event as VObjectEvent;

use libimagentryref::reference::Ref;
use libimagentryutil::isa::Is;
use libimagentryutil::isa::IsKindHeaderPathProvider;
use libimagstore::store::Entry;

use error::CalendarError as CE;
use error::CalendarErrorKind as CEK;
use error::*;

provide_kindflag_path!(pub IsEvent, "calendar.is_event");

/// A Event is a Entry in the store which refers to a calendar file that contains said Event.
pub trait Event : Ref {
    fn is_event(&self) -> Result<bool>;

    /// get the UID for this event.
    ///
    /// This information is stored in the Header
    fn get_uid(&self) -> Result<Option<String>>;

    // Accessing the actual icalendar file

    fn get_calendar(&self)    -> Result<ICalendar>;

    fn get_start(&self)       -> Result<NaiveDateTime>;
    fn get_end(&self)         -> Result<NaiveDateTime>;
    fn get_location(&self)    -> Result<String>;
    fn get_categories(&self)  -> Result<Vec<String>>;
    fn get_description(&self) -> Result<String>;
}

/// Helper macro for finding an ID and executing something for it
///
/// Not possible as a function because of lifetimes
macro_rules! return_for_id {
    ($this:ident, $doblock:expr) => {{
        let path = $this.get_path()?;
        let uid  = $this.get_uid()?.ok_or_else(|| CEK::EventWithoutUid(path.clone()))?;

        for event in $this.get_calendar()?.events() {
            let event    = event.map_err(|_| CE::from_kind(CEK::NotAnEvent(path.clone())))?;
            let event_id = event.get_uid()
                .ok_or_else(|| CE::from_kind(CEK::EventWithoutUid(path.clone())))?;

            if *event_id.raw() == uid {
                return $doblock(event, uid)
            }
        }

        Err(CE::from(CEK::CannotFindEventForId(uid)))
    }};
}

impl Event for Entry {
    fn is_event(&self) -> Result<bool> {
        self.is::<IsEvent>().map_err(From::from)
    }

    fn get_uid(&self) -> Result<Option<String>> {
        match self.get_header().read("calendar.event.uid").map_err(CE::from)? {
            None                        => Ok(None),
            Some(&Value::String(ref s)) => Ok(Some(s.clone())),
            Some(_) => Err(CEK::HeaderTypeError("calendar.event.uid", "String").into()),
        }
    }

    // Accessing the actual icalendar file

    fn get_calendar<'a>(&self) -> Result<ICalendar> {
        self.get_path()
            .map_err(CE::from)
            .and_then(::util::readfile)
            .and_then(|s| ICalendar::build(&s).map_err(CE::from))
    }

    fn get_start(&self) -> Result<NaiveDateTime> {
        return_for_id!(self, |ev: VObjectEvent, uid: String| {
            let dtstart = ev.get_dtstart()
                .ok_or_else(|| CE::from(CEK::EventMetadataMissing("start", uid.clone())))?;
            NaiveDateTime::parse_from_str(dtstart.raw(), "%Y%m%dT%H%M%S").map_err(CE::from)
        })
    }

    fn get_end(&self) -> Result<NaiveDateTime> {
        return_for_id!(self, |ev: VObjectEvent, uid: String| {
            let dtend = ev.get_dtend()
                .ok_or_else(|| CE::from(CEK::EventMetadataMissing("end", uid.clone())))?;
            NaiveDateTime::parse_from_str(dtend.raw(), "%Y%m%dT%H%M%S").map_err(CE::from)
        })
    }

    fn get_location(&self) -> Result<String> {
        return_for_id!(self, |ev: VObjectEvent, uid: String| {
            ev.get_location()
                .map(|l| l.raw().clone())
                .ok_or_else(|| CE::from(CEK::EventMetadataMissing("location", uid.clone())))
        })
    }

    fn get_categories(&self) -> Result<Vec<String>> {
        return_for_id!(self, |ev: VObjectEvent, uid: String| {
            ev.get_categories()
                .ok_or_else(|| CE::from(CEK::EventMetadataMissing("categories", uid.clone())))
                .map(|c| vec![c.raw().clone()])
                // TODO: vobject::icalendar::Event::get_categories() -> Option<Categories>
                // The API of vobject does not yet return categories split up.
        })
    }

    fn get_description(&self) -> Result<String> {
        return_for_id!(self, |ev: VObjectEvent, uid: String| {
            ev.get_description()
                .map(|c| c.raw().clone())
                .ok_or_else(|| CE::from(CEK::EventMetadataMissing("description", uid.clone())))
        })
    }

}

