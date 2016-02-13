use std::process::exit;

use libimagrt::runtime::Runtime;
use libimagutil::trace::trace_error;
use libimagcounter::counter::Counter;

pub fn delete(rt: &Runtime) {
    rt.cli()
        .subcommand_matches("delete")
        .map(|scmd| {
            debug!("Found 'delete' subcommand...");

            let name = String::from(scmd.value_of("name").unwrap()); // safe because clap enforces

            if let Err(e) = Counter::delete(name, rt.store()) {
                trace_error(&e);
                exit(1);
            }

            info!("Ok");
        });
}

