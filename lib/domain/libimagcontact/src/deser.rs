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

use vobject::vcard::Vcard;

/// A type which can be build from a Vcard and be serialized.
///
/// # Details
///
/// Deserializing is not supported by libimagcontact yet
/// Elements which are "empty" (as in empty list) or optional and not present are not serialized.
///
#[derive(Serialize, Debug)]
pub struct DeserVcard {

    #[serde(skip_serializing_if = "Vec::is_empty")]
    adr          : Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    anniversary  : Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bday         : Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    categories   : Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    clientpidmap : Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    email        : Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    fullname     : Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    gender       : Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    geo          : Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    impp         : Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    key          : Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    lang         : Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    logo         : Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    member       : Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name         : Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    nickname     : Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    note         : Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    org          : Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    photo        : Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    proid        : Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    related      : Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    rev          : Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    role         : Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    sound        : Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    tel          : Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    title        : Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    tz           : Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    uid          : Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    url          : Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    version      : Option<String>
}

impl From<Vcard> for DeserVcard {
    fn from(card: Vcard) -> DeserVcard {
        macro_rules! arystr {
            ($v:expr) => {
                $v.into_iter().map(|o| o.raw().clone()).collect()
            };
        };
        macro_rules! optstr {
            ($o:expr) => {
                $o.map(|o| o.raw().clone())
            };
        };

        DeserVcard {
            adr          : arystr!(card.adr()),
            anniversary  : optstr!(card.anniversary()),
            bday         : optstr!(card.bday()),
            categories   : arystr!(card.categories()),
            clientpidmap : optstr!(card.clientpidmap()),
            email        : arystr!(card.email()),
            fullname     : arystr!(card.fullname()),
            gender       : optstr!(card.gender()),
            geo          : arystr!(card.geo()),
            impp         : arystr!(card.impp()),
            key          : arystr!(card.key()),
            lang         : arystr!(card.lang()),
            logo         : arystr!(card.logo()),
            member       : arystr!(card.member()),
            name         : optstr!(card.name()),
            nickname     : arystr!(card.nickname()),
            note         : arystr!(card.note()),
            org          : arystr!(card.org()),
            photo        : arystr!(card.photo()),
            proid        : optstr!(card.proid()),
            related      : arystr!(card.related()),
            rev          : optstr!(card.rev()),
            role         : arystr!(card.role()),
            sound        : arystr!(card.sound()),
            tel          : arystr!(card.tel()),
            title        : arystr!(card.title()),
            tz           : arystr!(card.tz()),
            uid          : optstr!(card.uid()),
            url          : arystr!(card.url()),
            version      : optstr!(card.version()),
        }
    }
}

