use std::str::FromStr;

use libimagrt::runtime::Runtime;
use libimagerror::trace::trace_error_exit;
use libimagcounter::counter::Counter;
use libimagcounter::counter::CounterUnit;

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

            let unit = scmd
                .value_of("unit")
                .map(CounterUnit::new); 

            Counter::new(rt.store(), String::from(name), init)
                .and_then(|c| c.with_unit(unit))
                .unwrap_or_else(|e| {
                    warn!("Could not create Counter '{}' with initial value '{}'", name, init);
                    trace_error_exit(&e, 1);
                });
        });
}
