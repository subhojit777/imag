//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015, 2016 Matthias Beyer <mail@beyermatthias.de> and contributors
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

extern crate semver;
extern crate clap;
extern crate toml;
extern crate url;
#[macro_use] extern crate log;
#[macro_use] extern crate version;

extern crate libimagrt;
extern crate libimagmail;
extern crate libimagerror;
extern crate libimagutil;
extern crate libimagentryref;

use libimagerror::trace::{MapErrTrace, trace_error, trace_error_exit};
use libimagmail::mail::Mail;
use libimagentryref::reference::Ref;
use libimagrt::runtime::Runtime;
use libimagrt::setup::generate_runtime_setup;
use libimagutil::debug_result::*;
use libimagutil::info_result::*;

mod ui;

use ui::build_ui;

fn main() {
    let rt = generate_runtime_setup("imag-mail",
                                    &version!()[..],
                                    "Mail collection tool",
                                    build_ui);

    rt.cli()
        .subcommand_name()
        .map(|name| {
            debug!("Call {}", name);
            match name {
                "import-mail" => import_mail(&rt),
                "list"        => list(&rt),
                "mail-store"  => mail_store(&rt),
                _             => debug!("Unknown command") // More error handling
            }
        });
}

fn import_mail(rt: &Runtime) {
    let scmd = rt.cli().subcommand_matches("import-mail").unwrap();
    let path = scmd.value_of("path").unwrap(); // enforced by clap

    Mail::import_from_path(rt.store(), path)
        .map_err_trace()
        .map_info_str("Ok");
}

fn list(rt: &Runtime) {
    use libimagmail::error::MailErrorKind as MEK;
    use libimagmail::error::MapErrInto;

    let scmd = rt.cli().subcommand_matches("list").unwrap();
    let do_check_dead            = scmd.is_present("check-dead");
    let do_check_changed         = scmd.is_present("check-changed");
    let do_check_changed_content = scmd.is_present("check-changed-content");
    let do_check_changed_permiss = scmd.is_present("check-changed-permissions");
    let store = rt.store();

    let iter = match store.retrieve_for_module("ref") {
        Ok(iter) => iter.filter_map(|id| {
            Ref::get(store, id)
                .map_err_into(MEK::RefHandlingError)
                .and_then(|rf| Mail::from_ref(rf))
                .map_err_trace()
                .ok()
        }),
        Err(e)   => trace_error_exit(&e, 1),
    };

    fn list_mail(m: Mail) {
        let id = match m.get_message_id() {
            Ok(Some(f)) => f,
            Ok(None) => "<no id>".to_owned(),
            Err(e) => {
                trace_error(&e);
                "<error>".to_owned()
            },
        };

        let from = match m.get_from() {
            Ok(Some(f)) => f,
            Ok(None) => "<no from>".to_owned(),
            Err(e) => {
                trace_error(&e);
                "<error>".to_owned()
            },
        };

        let to = match m.get_to() {
            Ok(Some(f)) => f,
            Ok(None) => "<no to>".to_owned(),
            Err(e) => {
                trace_error(&e);
                "<error>".to_owned()
            },
        };

        let subject = match m.get_subject() {
            Ok(Some(f)) => f,
            Ok(None) => "<no subject>".to_owned(),
            Err(e) => {
                trace_error(&e);
                "<error>".to_owned()
            },
        };

        println!("Mail: {id}\n\tFrom: {from}\n\tTo: {to}\n\t{subj}\n",
                 from = from,
                 id   = id,
                 subj = subject,
                 to   = to
        );
    }

    // TODO: Implement lister type in libimagmail for this
    for mail in iter {
        list_mail(mail)
    }
}

fn mail_store(rt: &Runtime) {
    let scmd = rt.cli().subcommand_matches("mail-store").unwrap();
    error!("This feature is currently not implemented.");
    unimplemented!()
}

