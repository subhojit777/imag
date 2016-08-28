use std::path::PathBuf;

use clap::{Arg, ArgMatches};

use libimagstore::storeid::StoreId;
use libimagerror::into::IntoError;

use result::Result;
use error::MapErrInto;
use error::EntrySelectErrorKind as ESEK;

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
        .ok_or(ESEK::IdMissingError.into_error())
        .map_err_into(ESEK::CLIError)
        .and_then(|vals| {
            vals.into_iter()
                .fold(Ok(vec![]), |acc, elem| {
                    acc.and_then(|mut v| {
                        let elem = StoreId::new_baseless(PathBuf::from(String::from(elem)));
                        let elem = try!(elem.map_err_into(ESEK::StoreIdParsingError));
                        v.push(elem);
                        Ok(v)
                    })
                })
        })
}

pub fn get_or_select_id(matches: &ArgMatches, store_path: &PathBuf) -> Result<Vec<StoreId>> {
    use interactor::{pick_file, default_menu_cmd};

    match get_id(matches).map_err_into(ESEK::IdSelectingError) {
        Ok(v) => Ok(v),
        Err(_) => {
            let path = store_path.clone();
            let p  = try!(pick_file(default_menu_cmd, path).map_err_into(ESEK::IdSelectingError));
            let id = try!(StoreId::new_baseless(p).map_err_into(ESEK::StoreIdParsingError));
            Ok(vec![id])
        },
    }
}

