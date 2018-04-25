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

use libimagcontact::deser::DeserVcard;

pub fn build_data_object_for_handlebars<'a>(i: usize, vcard: &DeserVcard) -> BTreeMap<&'static str, String> {
    let mut data = BTreeMap::new();

    let process_list = |list: &Vec<String>| {
        list.iter()
            .map(String::clone)
            .collect::<Vec<_>>()
            .join(", ")
    };

    let process_opt  = |opt: Option<&String>| {
        opt.map(String::clone).unwrap_or_else(String::new)
    };

    {
        data.insert("i"            , format!("{}", i));

        // The hash (as in libimagentryref) of the contact
        data.insert("id"           , process_opt(vcard.uid()));
        data.insert("ADR"          , process_list(vcard.adr()));
        data.insert("ANNIVERSARY"  , process_opt(vcard.anniversary()));
        data.insert("BDAY"         , process_opt(vcard.bday()));
        data.insert("CATEGORIES"   , process_list(vcard.categories()));
        data.insert("CLIENTPIDMAP" , process_opt(vcard.clientpidmap()));
        data.insert("EMAIL"        , process_list(vcard.email()));
        data.insert("FN"           , process_list(vcard.fullname()));
        data.insert("GENDER"       , process_opt(vcard.gender()));
        data.insert("GEO"          , process_list(vcard.geo()));
        data.insert("IMPP"         , process_list(vcard.impp()));
        data.insert("KEY"          , process_list(vcard.key()));
        data.insert("LANG"         , process_list(vcard.lang()));
        data.insert("LOGO"         , process_list(vcard.logo()));
        data.insert("MEMBER"       , process_list(vcard.member()));
        data.insert("N"            , process_opt(vcard.name()));
        data.insert("NICKNAME"     , process_list(vcard.nickname()));
        data.insert("NOTE"         , process_list(vcard.note()));
        data.insert("ORG"          , process_list(vcard.org()));
        data.insert("PHOTO"        , process_list(vcard.photo()));
        data.insert("PRIOD"        , process_opt(vcard.proid()));
        data.insert("RELATED"      , process_list(vcard.related()));
        data.insert("REV"          , process_opt(vcard.rev()));
        data.insert("ROLE"         , process_list(vcard.role()));
        data.insert("SOUND"        , process_list(vcard.sound()));
        data.insert("TEL"          , process_list(vcard.tel()));
        data.insert("TITLE"        , process_list(vcard.title()));
        data.insert("TZ"           , process_list(vcard.tz()));
        data.insert("UID"          , process_opt(vcard.uid()));
        data.insert("URL"          , process_list(vcard.url()));
        data.insert("VERSION"      , process_opt(vcard.version()));
    }

    data
}

