use clap::App;

use runtime::Runtime;

pub type Name          = &'static str;
pub type Version<'a>   = &'a str;
pub type About         = &'static str;

/// Helper to generate the Runtime object
///
/// exit()s the program if the runtime couldn't be build, prints error with println!() before
/// exiting
pub fn generate_runtime_setup<'a, B>(name: Name, version: Version<'a>, about: About, builder: B)
    -> Runtime<'a>
    where B: FnOnce(App<'a, 'a>) -> App<'a, 'a>
{
    use std::process::exit;
    use libimagerror::trace::trace_error_dbg;

    Runtime::new(builder(Runtime::get_default_cli_builder(name, version, about)))
        .unwrap_or_else(|e| {
            println!("Could not set up Runtime");
            println!("{:?}", e);
            trace_error_dbg(&e);
            exit(1);
        })
}
