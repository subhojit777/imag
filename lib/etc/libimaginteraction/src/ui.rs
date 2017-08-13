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

use std::path::PathBuf;

use clap::{Arg, ArgMatches};

use libimagstore::storeid::StoreId;
use libimagerror::into::IntoError;

use result::Result;
use error::MapErrInto;
use error::InteractionErrorKind as IEK;

pub fn id_argument<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name(id_argument_name())
        .short(id_argument_short())
        .long(id_argument_long())
        .takes_value(true)
        .multiple(true)
        .help("Specify the Store-Id")
}

pub fn id_argument_name() -> &'static str {
    "id-argument"
}

pub fn id_argument_short() -> &'static str {
    "i"
}

pub fn id_argument_long() -> &'static str {
    "id"
}

pub fn get_id(matches: &ArgMatches) -> Result<Vec<StoreId>> {
    matches
        .values_of(id_argument_name())
        .ok_or(IEK::IdMissingError.into_error())
        .map_err_into(IEK::CLIError)
        .and_then(|vals| {
            vals.into_iter()
                .fold(Ok(vec![]), |acc, elem| {
                    acc.and_then(|mut v| {
                        let elem = StoreId::new_baseless(PathBuf::from(String::from(elem)));
                        let elem = try!(elem.map_err_into(IEK::StoreIdParsingError));
                        v.push(elem);
                        Ok(v)
                    })
                })
        })
}

pub fn get_or_select_id(matches: &ArgMatches, store_path: &PathBuf) -> Result<Vec<StoreId>> {
    use interactor::{pick_file, default_menu_cmd};

    match get_id(matches).map_err_into(IEK::IdSelectingError) {
        Ok(v) => Ok(v),
        Err(_) => {
            let path = store_path.clone();
            let p  = try!(pick_file(default_menu_cmd, path).map_err_into(IEK::IdSelectingError));
            let id = try!(StoreId::new_baseless(p).map_err_into(IEK::StoreIdParsingError));
            Ok(vec![id])
        },
    }
}

