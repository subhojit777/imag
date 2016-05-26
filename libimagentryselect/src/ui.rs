use clap::{Arg, ArgMatches};

use libimagstore::storeid::StoreId;

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

