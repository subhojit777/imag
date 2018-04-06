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

use std::collections::BTreeMap;
use vobject::vcard::Vcard;

pub fn build_data_object_for_handlebars<'a>(i: usize, hash: String, vcard: &Vcard) -> BTreeMap<&'static str, String> {
    let mut data = BTreeMap::new();
    {
        data.insert("i"            , format!("{}", i));

        // The hash (as in libimagentryref) of the contact
        data.insert("id"           , hash);

        data.insert("ADR"          , vcard.adr()
                    .into_iter().map(|c| c.raw().clone()).collect::<Vec<_>>().join(", "));

        data.insert("ANNIVERSARY"  , vcard.anniversary()
                .map(|c| c.raw().clone()).unwrap_or(String::new()));

        data.insert("BDAY"         , vcard.bday()
                    .map(|c| c.raw().clone()).unwrap_or(String::new()));

        data.insert("CATEGORIES"   , vcard.categories()
                    .into_iter().map(|c| c.raw().clone()).collect::<Vec<_>>().join(", "));

        data.insert("CLIENTPIDMAP" , vcard.clientpidmap()
                    .map(|c| c.raw().clone()).unwrap_or(String::new()));

        data.insert("EMAIL"        , vcard.email()
                    .into_iter().map(|c| c.raw().clone()).collect::<Vec<_>>().join(", "));

        data.insert("FN"           , vcard.fullname()
                    .into_iter().map(|c| c.raw().clone()).collect::<Vec<_>>().join(", "));

        data.insert("GENDER"       , vcard.gender()
                    .map(|c| c.raw().clone()).unwrap_or(String::new()));

        data.insert("GEO"          , vcard.geo()
                    .into_iter().map(|c| c.raw().clone()).collect::<Vec<_>>().join(", "));

        data.insert("IMPP"         , vcard.impp()
                    .into_iter().map(|c| c.raw().clone()).collect::<Vec<_>>().join(", "));

        data.insert("KEY"          , vcard.key()
                    .into_iter().map(|c| c.raw().clone()).collect::<Vec<_>>().join(", "));

        data.insert("LANG"         , vcard.lang()
                    .into_iter().map(|c| c.raw().clone()).collect::<Vec<_>>().join(", "));

        data.insert("LOGO"         , vcard.logo()
                    .into_iter().map(|c| c.raw().clone()).collect::<Vec<_>>().join(", "));

        data.insert("MEMBER"       , vcard.member()
                    .into_iter().map(|c| c.raw().clone()).collect::<Vec<_>>().join(", "));

        data.insert("N"            , vcard.name()
                    .map(|c| c.raw().clone()).unwrap_or(String::new()));

        data.insert("NICKNAME"     , vcard.nickname()
                    .into_iter().map(|c| c.raw().clone()).collect::<Vec<_>>().join(", "));

        data.insert("NOTE"         , vcard.note()
                    .into_iter().map(|c| c.raw().clone()).collect::<Vec<_>>().join(", "));

        data.insert("ORG"          , vcard.org()
                    .into_iter().map(|c| c.raw().clone()).collect::<Vec<_>>().join(", "));

        data.insert("PHOTO"        , vcard.photo()
                    .into_iter().map(|c| c.raw().clone()).collect::<Vec<_>>().join(", "));

        data.insert("PRIOD"        , vcard.proid()
                    .map(|c| c.raw().clone()).unwrap_or(String::new()));

        data.insert("RELATED"      , vcard.related()
                    .into_iter().map(|c| c.raw().clone()).collect::<Vec<_>>().join(", "));

        data.insert("REV"          , vcard.rev()
                    .map(|c| c.raw().clone()).unwrap_or(String::new()));

        data.insert("ROLE"         , vcard.role()
                    .into_iter().map(|c| c.raw().clone()).collect::<Vec<_>>().join(", "));

        data.insert("SOUND"        , vcard.sound()
                    .into_iter().map(|c| c.raw().clone()).collect::<Vec<_>>().join(", "));

        data.insert("TEL"          , vcard.tel()
                    .into_iter().map(|c| c.raw().clone()).collect::<Vec<_>>().join(", "));

        data.insert("TITLE"        , vcard.title()
                    .into_iter().map(|c| c.raw().clone()).collect::<Vec<_>>().join(", "));

        data.insert("TZ"           , vcard.tz()
                    .into_iter().map(|c| c.raw().clone()).collect::<Vec<_>>().join(", "));

        data.insert("UID"          , vcard.uid()
                    .map(|c| c.raw().clone()).unwrap_or(String::new()));

        data.insert("URL"          , vcard.url()
                    .into_iter().map(|c| c.raw().clone()).collect::<Vec<_>>().join(", "));

        data.insert("VERSION"      , vcard.version()
                    .map(|c| c.raw().clone()).unwrap_or(String::new()));
    }

    data
}

