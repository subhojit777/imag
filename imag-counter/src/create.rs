use std::str::FromStr;
use std::process::exit;

use libimagrt::runtime::Runtime;
use libimagutil::trace::trace_error;
use libimagcounter::counter::Counter;

pub fn create(rt: &Runtime) {
    rt.cli()
        .subcommand_matches("create")
        .map(|scmd| {
            debug!("Found 'create' subcommand...");

            let name = scmd.value_of("name").unwrap(); // safe because clap enforces
            let init : i64 = scmd
                .value_of("initval")
                .and_then(|i| FromStr::from_str(i).ok())
                .unwrap_or(0);

            match Counter::new(rt.store(), String::from(name.clone()), init) {
                Err(e) => {
                    warn!("Could not create Counter '{}' with initial value '{}'", name, init);
                    trace_error(&e);
                    exit(1);
                },
                Ok(_) => info!("Created Counter '{}' with initial value '{}'", name, init),
            }
        });
}
