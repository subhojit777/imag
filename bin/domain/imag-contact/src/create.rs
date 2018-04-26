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

#![deny(
    non_camel_case_types,
    non_snake_case,
    path_statements,
    trivial_numeric_casts,
    unstable_features,
    unused_allocation,
    unused_import_braces,
    unused_imports,
    unused_must_use,
    unused_mut,
    unused_qualifications,
    while_true,
)]

use std::collections::BTreeMap;
use std::process::exit;
use std::io::Write;
use std::path::PathBuf;
use std::fs::OpenOptions;

use vobject::vcard::Vcard;
use vobject::write_component;
use toml_query::read::TomlValueReadExt;
use toml::Value;
use uuid::Uuid;

use libimagcontact::error::ContactError as CE;
use libimagcontact::store::UniqueContactPathGenerator;
use libimagrt::runtime::Runtime;
use libimagerror::str::ErrFromStr;
use libimagerror::trace::MapErrTrace;
use libimagerror::trace::trace_error;
use libimagutil::warn_result::WarnResult;
use libimagentryref::refstore::RefStore;

const TEMPLATE : &'static str = include_str!("../static/new-contact-template.toml");

#[cfg(test)]
mod test {
    use toml::Value;
    use super::TEMPLATE;

    const TEMPLATE_WITH_DATA : &'static str = include_str!("../static/new-contact-template-test.toml");

    #[test]
    fn test_validity_template_toml() {
        let _ : Value = ::toml::de::from_str(TEMPLATE).unwrap();
    }

    #[test]
    fn test_validity_template_toml_without_comments() {
        let _ : Value = ::toml::de::from_str(TEMPLATE_WITH_DATA).unwrap();
    }
}

macro_rules! ask_continue {
    { yes => $yes:expr; no => $no:expr } => {
        if ::libimaginteraction::ask::ask_bool("Edit tempfile", Some(true)) {
            $yes
        } else {
            $no
        }
    };
}

pub fn create(rt: &Runtime) {
    let scmd         = rt.cli().subcommand_matches("create").unwrap();
    let mut template = String::from(TEMPLATE);

    let (mut dest, location, uuid) : (Box<Write>, Option<PathBuf>, String) = {
        if let Some(mut fl) = scmd.value_of("file-location").map(PathBuf::from) {
            let uuid = if fl.is_file() {
                error!("File does exist, cannot create/override");
                exit(1);
            } else if fl.is_dir() {
                let uuid = Uuid::new_v4().hyphenated().to_string();
                fl.push(uuid.clone());
                info!("Creating file: {:?}", fl);
                Some(uuid)
            } else {
                None
            };

            debug!("Destination = {:?}", fl);

            let file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(fl.clone())
                .map_warn_err_str("Cannot create/open destination File. Stopping.")
                .err_from_str()
                .map_err(CE::from)
                .map_err_trace_exit_unwrap(1);

            let uuid_string = uuid
                .unwrap_or_else(|| {
                    fl.file_name()
                        .and_then(|fname| fname.to_str())
                        .map(String::from)
                        .unwrap_or_else(|| {
                            error!("Cannot calculate UUID for vcard");
                            exit(1)
                        })
                });

            (Box::new(file), Some(fl), uuid_string)
        } else {
            // We generate a random uuid for stdout
            let uuid = Uuid::new_v4().hyphenated().to_string();
            (Box::new(rt.stdout()), None, uuid)
        }
    };

    loop {
        ::libimagentryedit::edit::edit_in_tmpfile(&rt, &mut template)
            .map_warn_err_str("Editing failed.")
            .map_err_trace_exit_unwrap(1);

        if template == TEMPLATE || template.is_empty() {
            error!("No (changed) content in tempfile. Not doing anything.");
            exit(2);
        }

        match ::toml::de::from_str(&template)
            .map(|toml| parse_toml_into_vcard(toml, uuid.clone()))
            .err_from_str()
            .map_err(CE::from)
        {
            Err(e) => {
                error!("Error parsing template");
                trace_error(&e);
                ask_continue! { yes => continue; no => exit(1) };
            },

            Ok(None)        => continue,
            Ok(Some(vcard)) => {
                if template == TEMPLATE || template.is_empty() {
                    if ::libimaginteraction::ask::ask_bool("Abort contact creating", Some(false)) {
                        exit(1);
                    } else {
                        continue;
                    }
                }

                let vcard_string = write_component(&vcard);
                let _ = dest
                    .write_all(&vcard_string.as_bytes())
                    .map_err(CE::from)
                    .map_err_trace_exit_unwrap(1);

                break;
            }
        }
    }

    if let Some(location) = location {
        if !scmd.is_present("dont-track") {
            RefStore::create_ref::<UniqueContactPathGenerator, _>(rt.store(), location)
                .map_err_trace_exit_unwrap(1);

            info!("Created entry in store");
        } else {
            info!("Not creating entry in store");
        }
    } else {
        info!("Cannot track stdout-created contact information");
    }

    info!("Ready");
}

fn parse_toml_into_vcard(toml: Value, uuid: String) -> Option<Vcard> {
    let mut vcard = Vcard::default().with_uid(uuid);

    { // parse name
        debug!("Parsing name");
        let firstname = read_str_from_toml(&toml, "name.first", true);
        trace!("firstname = {:?}", firstname);

        let lastname  = read_str_from_toml(&toml, "name.last", true);
        trace!("lastname = {:?}", lastname);

        vcard = vcard.with_name(parameters!(),
            read_str_from_toml(&toml, "name.prefix", false),
            firstname.clone(),
            read_str_from_toml(&toml, "name.additional", false),
            lastname.clone(),
            read_str_from_toml(&toml, "name.suffix", false));

        if let (Some(first), Some(last)) = (firstname, lastname) {
            trace!("Building fullname: '{} {}'", first, last);
            vcard = vcard.with_fullname(format!("{} {}", first, last));
        }
    }

    { // parse personal
        debug!("Parsing person information");
        let birthday = read_str_from_toml(&toml, "person.birthday", false);
        trace!("birthday = {:?}", birthday);

        if let Some(bday) = birthday {
            vcard = vcard.with_bday(parameters!(), bday);
        }
    }

    { // parse nicknames
        debug!("Parsing nicknames");
        match toml.read("nickname").map_err_trace_exit_unwrap(1) {
            Some(&Value::Array(ref ary)) => {
                for (i, element) in ary.iter().enumerate() {
                    let nicktype = match read_str_from_toml(element, "type", false) {
                        None    => BTreeMap::new(),
                        Some(p) => {
                            let mut m = BTreeMap::new();
                            m.insert("TYPE".into(), p);
                            m
                        },
                    };

                    let name = match read_str_from_toml(element, "name", false) {
                        Some(p) => p,
                        None    => {
                            error!("Key 'nickname.[{}].name' missing", i);
                            ask_continue! { yes => return None; no => exit(1) };
                        },
                    };

                    trace!("nick type = {:?}", nicktype);
                    trace!("name      = {:?}", name);

                    vcard = vcard.with_nickname(nicktype, name);
                }
            },

            Some(&Value::String(ref name)) => {
                vcard = vcard.with_nickname(parameters!(), name.clone());
            }

            Some(_) => {
                error!("Type Error: Expected Array or String at 'nickname'");
                ask_continue! { yes => return None; no => exit(1) };
            },
            None => {
                // nothing
            },
        }
    }

    { // parse organisation
        debug!("Parsing organisation");

        if let Some(orgs) = read_strary_from_toml(&toml, "organisation.name") {
            trace!("orgs = {:?}", orgs);
            vcard = vcard.with_org(orgs);
        }

        if let Some(title) = read_str_from_toml(&toml, "organisation.title", false) {
            trace!("title = {:?}", title);
            vcard = vcard.with_title(title);
        }

        if let Some(role) = read_str_from_toml(&toml, "organisation.role", false) {
            trace!("role = {:?}", role);
            vcard = vcard.with_role(role);
        }
    }

    { // parse phone
        debug!("Parse phone");
        match toml.read("person.phone").map_err_trace_exit_unwrap(1) {
            Some(&Value::Array(ref ary)) => {
                for (i, element) in ary.iter().enumerate() {
                    let phonetype = match read_str_from_toml(element, "type", false) {
                        Some(p) => p,
                        None => {
                            error!("Key 'phones.[{}].type' missing", i);
                            ask_continue! { yes => return None; no => exit(1) };
                        }
                    };

                    let number = match read_str_from_toml(element, "number", false) {
                        Some(p) => p,
                        None => {
                            error!("Key 'phones.[{}].number' missing", i);
                            ask_continue! { yes => return None; no => exit(1) };
                        }
                    };

                    trace!("phonetype = {:?}", phonetype);
                    trace!("number    = {:?}", number);

                    vcard = vcard.with_tel(parameters!("TYPE" => phonetype), number);
                }
            },

            Some(_) => {
                error!("Expected Array at 'phones'.");
                ask_continue! { yes => return None; no => exit(1) };
            },
            None => {
                // nothing
            },
        }
    }

    { // parse address
        debug!("Parsing address");
        match toml.read("addresses").map_err_trace_exit_unwrap(1) {
            Some(&Value::Array(ref ary)) => {
                for (i, element) in ary.iter().enumerate() {
                    let adrtype  = match read_str_from_toml(element, "type", false) {
                        None => {
                            error!("Key 'adresses.[{}].type' missing", i);
                            ask_continue! { yes => return None; no => exit(1) };
                        },
                        Some(p) => p,
                    };
                    trace!("adrtype = {:?}", adrtype);

                    let bx       = read_str_from_toml(element, "box", false);
                    let extended = read_str_from_toml(element, "extended", false);
                    let street   = read_str_from_toml(element, "street", false);
                    let code     = read_str_from_toml(element, "code", false);
                    let city     = read_str_from_toml(element, "city", false);
                    let region   = read_str_from_toml(element, "region", false);
                    let country  = read_str_from_toml(element, "country", false);

                    trace!("bx       = {:?}", bx);
                    trace!("extended = {:?}", extended);
                    trace!("street   = {:?}", street);
                    trace!("code     = {:?}", code);
                    trace!("city     = {:?}", city);
                    trace!("region   = {:?}", region);
                    trace!("country  = {:?}", country);

                    vcard = vcard.with_adr(
                        parameters!("TYPE" => adrtype),
                        bx, extended, street, code, city, region, country
                    );
                }
            },

            Some(_) => {
                error!("Type Error: Expected Array at 'addresses'");
                ask_continue! { yes => return None; no => exit(1) };
            },
            None => {
                // nothing
            },
        }
    }

    { // parse email
        debug!("Parsing email");
        match toml.read("person.email").map_err_trace_exit_unwrap(1) {
            Some(&Value::Array(ref ary)) => {
                for (i, element) in ary.iter().enumerate() {
                    let mailtype  = match read_str_from_toml(element, "type", false) {
                        None => {
                            error!("Error: 'email.[{}].type' missing", i);
                            ask_continue! { yes => return None; no => exit(1) };
                        },
                        Some(p) => p,
                    }; // TODO: Unused, because unsupported by vobject

                    let mail = match read_str_from_toml(element, "addr", false) {
                        None => {
                            error!("Error: 'email.[{}].addr' missing", i);
                            ask_continue! { yes => return None; no => exit(1) };
                        },
                        Some(p) => p,
                    };

                    trace!("mailtype = {:?} (UNUSED)", mailtype);
                    trace!("mail     = {:?}", mail);

                    vcard = vcard.with_email(mail);
                }
            },

            Some(_) => {
                error!("Type Error: Expected Array at 'email'");
                ask_continue! { yes => return None; no => exit(1) };
            },
            None => {
                // nothing
            },
        }
    }

    { // parse others
        debug!("Parsing others");
        if let Some(categories) = read_strary_from_toml(&toml, "other.categories") {
            vcard = vcard.with_categories(categories);
        } else {
            debug!("No categories");
        }

        if let Some(webpage) = read_str_from_toml(&toml, "other.webpage", false) {
            vcard = vcard.with_url(webpage);
        } else {
            debug!("No webpage");
        }

        if let Some(note) = read_str_from_toml(&toml, "other.note", false) {
            vcard = vcard.with_note(note);
        } else {
            debug!("No note");
        }

    }

    Some(vcard)
}

fn read_strary_from_toml(toml: &Value, path: &'static str) -> Option<Vec<String>> {
    match toml.read(path).map_warn_err_str(&format!("Failed to read value at '{}'", path)) {
        Ok(Some(&Value::Array(ref vec))) => {
            let mut v = Vec::new();
            for elem in vec {
                match *elem {
                    Value::String(ref s) => v.push(s.clone()),
                    _ => {
                        error!("Type Error: '{}' must be Array<String>", path);
                        return None
                    },
                }
            }

            Some(v)
        }
        Ok(Some(&Value::String(ref s))) => {
            warn!("Having String, wanting Array<String> ... going to auto-fix");
            Some(vec![s.clone()])
        },
        Ok(Some(_)) => {
            error!("Type Error: '{}' must be Array<String>", path);
            None
        },
        Ok(None) => None,
        Err(_) => None,
    }
}

fn read_str_from_toml(toml: &Value, path: &'static str, must_be_there: bool) -> Option<String> {
    let v = toml.read(path)
        .map_warn_err_str(&format!("Failed to read value at '{}'", path));

    match v {
        Ok(Some(&Value::String(ref s))) => Some(s.clone()),
        Ok(Some(_)) => {
            error!("Type Error: '{}' must be String", path);
            None
        },
        Ok(None) => {
            if must_be_there {
                error!("Expected '{}' to be present, but is not.", path);
            }
            None
        },
        Err(e) => {
            trace_error(&e);
            None
        }
    }
}

#[cfg(test)]
mod test_parsing {
    use super::parse_toml_into_vcard;

    // TODO
    const TEMPLATE : &'static str = include_str!("../static/new-contact-template-test.toml");

    #[test]
    fn test_template_names() {
        let uid = String::from("uid");
        let vcard = parse_toml_into_vcard(::toml::de::from_str(TEMPLATE).unwrap(), uid);
        assert!(vcard.is_some(), "Failed to parse test template.");
        let vcard = vcard.unwrap();

        assert!(vcard.name().is_some());

        assert_eq!(vcard.uid().unwrap().raw(), "uid");
        assert_eq!(vcard.name().unwrap().surname().unwrap(), "test");
        assert_eq!(vcard.name().unwrap().given_name().unwrap(), "test");
        assert_eq!(vcard.name().unwrap().additional_names().unwrap(), "test");
        assert_eq!(vcard.name().unwrap().honorific_prefixes().unwrap(), "test");
        assert_eq!(vcard.name().unwrap().honorific_suffixes().unwrap(), "test");
    }

    #[test]
    fn test_template_person() {
        let uid = String::from("uid");
        let vcard = parse_toml_into_vcard(::toml::de::from_str(TEMPLATE).unwrap(), uid);
        assert!(vcard.is_some(), "Failed to parse test template.");
        let vcard = vcard.unwrap();

        assert!(vcard.bday().is_some());

        assert_eq!(vcard.bday().unwrap().raw(), "2017-01-01");

        assert_eq!(vcard.nickname().len(), 1);
        assert_eq!(vcard.nickname()[0].raw(), "boss");

        // TODO: parameters() not yet implemented in underlying API
        // assert!(vcard.nickname()[0].parameters().contains_key("work"));
    }

    #[test]
    fn test_template_organization() {
        let uid = String::from("uid");
        let vcard = parse_toml_into_vcard(::toml::de::from_str(TEMPLATE).unwrap(), uid);
        assert!(vcard.is_some(), "Failed to parse test template.");
        let vcard = vcard.unwrap();

        assert_eq!(vcard.org().len(), 1);
        assert_eq!(vcard.org()[0].raw(), "test");

        assert_eq!(vcard.title().len(), 1);
        assert_eq!(vcard.title()[0].raw(), "test");

        assert_eq!(vcard.role().len(), 1);
        assert_eq!(vcard.role()[0].raw(), "test");
    }

    #[test]
    fn test_template_phone() {
        let uid = String::from("uid");
        let vcard = parse_toml_into_vcard(::toml::de::from_str(TEMPLATE).unwrap(), uid);
        assert!(vcard.is_some(), "Failed to parse test template.");
        let vcard = vcard.unwrap();

        assert_eq!(vcard.tel().len(), 1);
        assert_eq!(vcard.tel()[0].raw(), "0123 123456789");

        // TODO: parameters() not yet implemented in underlying API
        // assert!(vcard.tel()[0].parameters().contains_key("type"));
        // assert_eq!(vcard.tel()[0].parameters().get("type").unwrap(), "home");
    }

    #[test]
    fn test_template_email() {
        let uid = String::from("uid");
        let vcard = parse_toml_into_vcard(::toml::de::from_str(TEMPLATE).unwrap(), uid);
        assert!(vcard.is_some(), "Failed to parse test template.");
        let vcard = vcard.unwrap();

        assert_eq!(vcard.email().len(), 1);
        assert_eq!(vcard.email()[0].raw(), "examle@examplemail.org");

        // TODO: parameters() not yet implemented in underlying API
        // assert!(vcard.email()[0].parameters().contains_key("type"));
        // assert_eq!(vcard.email()[0].parameters().get("type").unwrap(), "home");
    }

    #[test]
    fn test_template_addresses() {
        let uid = String::from("uid");
        let vcard = parse_toml_into_vcard(::toml::de::from_str(TEMPLATE).unwrap(), uid);
        assert!(vcard.is_some(), "Failed to parse test template.");
        let vcard = vcard.unwrap();

        assert_eq!(vcard.adr().len(), 1);
        assert_eq!(vcard.adr()[0].raw(), "testbox;testextended;teststreet;testcode;testcity;testregion;testcountry");

        // TODO: parameters() not yet implemented in underlying API
        //for e in &["box", "extended", "street", "code", "city", "region", "country"] {
        //    assert!(vcard.adr()[0].parameters().contains_key(e));
        //    assert_eq!(vcard.adr()[0].parameters().get(e).unwrap(), "test");
        //}
    }

    #[test]
    fn test_template_other() {
        let uid = String::from("uid");
        let vcard = parse_toml_into_vcard(::toml::de::from_str(TEMPLATE).unwrap(), uid);
        assert!(vcard.is_some(), "Failed to parse test template.");
        let vcard = vcard.unwrap();

        assert_eq!(vcard.categories().len(), 1);
        assert_eq!(vcard.categories()[0].raw(), "test");

        assert_eq!(vcard.url().len(), 1);
        assert_eq!(vcard.url()[0].raw(), "test");

        assert_eq!(vcard.note().len(), 1);
        assert_eq!(vcard.note()[0].raw(), "test");
    }
}

