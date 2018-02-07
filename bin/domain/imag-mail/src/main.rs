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

extern crate clap;
#[macro_use] extern crate log;

extern crate libimagrt;
extern crate libimagmail;
extern crate libimagerror;
extern crate libimagutil;

use libimagerror::trace::{MapErrTrace, trace_error, trace_error_exit};
use libimagmail::mail::Mail;
use libimagrt::runtime::Runtime;
use libimagrt::setup::generate_runtime_setup;
use libimagutil::info_result::*;

mod ui;

use ui::build_ui;

fn main() {
    let rt = generate_runtime_setup("imag-mail",
                                    env!("CARGO_PKG_VERSION"),
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

    let _ = Mail::import_from_path(rt.store(), path)
        .map_err_trace()
        .map_info_str("Ok");
}

fn list(rt: &Runtime) {
    use libimagmail::error::MailErrorKind as MEK;
    use libimagmail::error::ResultExt;

    let store = rt.store();

    let iter = match store.retrieve_for_module("ref") {
        Ok(iter) => iter.filter_map(|id| {
            match store.get(id).chain_err(|| MEK::RefHandlingError).map_err_trace() {
                Ok(Some(fle)) => Mail::from_fle(fle).map_err_trace().ok(),
                Ok(None)      => None,
                Err(e)        => trace_error_exit(&e, 1),
            }
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
    let _ = rt.cli().subcommand_matches("mail-store").unwrap();
    error!("This feature is currently not implemented.");
    unimplemented!()
}

