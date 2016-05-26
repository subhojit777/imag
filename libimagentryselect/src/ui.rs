use std::path::PathBuf;

use clap::{Arg, ArgMatches};

use libimagstore::storeid::StoreId;
use libimagerror::trace::trace_error;

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

pub fn get_id(matches: &ArgMatches) -> Option<Vec<StoreId>> {
    matches.values_of(id_argument_name())
        .map(|vals| {
            vals.into_iter()
                .map(String::from)
                 .map(StoreId::from)
                 .collect()
        })
}

pub fn get_or_select_id(matches: &ArgMatches, store_path: &PathBuf) -> Option<Vec<StoreId>> {
    use interactor::{pick_file, default_menu_cmd};

    get_id(matches).or_else(|| {
        match pick_file(default_menu_cmd, store_path.clone()) {
            Err(e) => {
                trace_error(&e);
                None
            },

            Ok(p) => Some(vec![StoreId::from(p)]),
        }
    })
}

