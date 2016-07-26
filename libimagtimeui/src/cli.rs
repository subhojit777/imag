use clap::Arg;

pub fn build_datetime_cli_component<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name(datetime_arg_name())
        .short(datetime_arg_short())
        .long(datetime_arg_long())
        .takes_value(true)
        .multiple(false)
        .help("Specify a DateTime")
}

pub fn datetime_arg_name() -> &'static str {
    "datetime"
}

pub fn datetime_arg_long() -> &'static str {
    "datetime"
}

pub fn datetime_arg_short() -> &'static str {
    "T"
}

