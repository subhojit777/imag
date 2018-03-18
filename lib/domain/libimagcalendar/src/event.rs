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

use chrono::NaiveDateTime;

use toml::Value;
use toml_query::read::TomlValueReadExt;
use vobject::icalendar::ICalendar;

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

    fn get_start(&self)       -> Result<NaiveDateTime>;
    fn get_end(&self)         -> Result<NaiveDateTime>;
    fn get_location(&self)    -> Result<String>;
    fn get_categories(&self)  -> Result<Vec<String>>;
    fn get_description(&self) -> Result<String>;
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

    fn get_start(&self) -> Result<NaiveDateTime> {
        let path = self.get_path()?;
        let uid  = self.get_uid()?.ok_or_else(|| CEK::EventWithoutUid(path.clone()))?;

        self.get_path()
            .map_err(CE::from)
            .and_then(::util::readfile)
            .and_then(|s| ICalendar::build(&s).map_err(CE::from))?
            .events()
            .map(|ev| ev.map_err(|_| CEK::NotAnEvent(path.clone()).into()))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .map(|ev| {
                ev.get_uid()
                    .ok_or_else(|| CEK::EventWithoutUid(path.clone()).into())
                    .map(|id| (ev, *id.raw() == uid))
            })
            .filter(|res| match res {
                &Ok((_, boo)) => boo,
                _ => true,
            }) // uid match or error
            .map(|res| res.map(|tpl| tpl.0))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .next()
            .ok_or_else(|| CE::from(CEK::CannotFindEventForId(uid.clone())))?
            .get_dtstart()
            .ok_or_else(|| CE::from(CEK::EventMetadataMissing("start", uid.clone())))
            .and_then(|dtstart| {
                NaiveDateTime::parse_from_str(dtstart.raw(), "%Y%m%dT%H%M%S").map_err(CE::from)
            })
    }

    fn get_end(&self) -> Result<NaiveDateTime> {
        unimplemented!()
    }

    fn get_location(&self) -> Result<String> {
        unimplemented!()
    }

    fn get_categories(&self) -> Result<Vec<String>> {
        unimplemented!()
    }

    fn get_description(&self) -> Result<String> {
        unimplemented!()
    }

}
