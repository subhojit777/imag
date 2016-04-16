use libimagrt::runtime::Runtime;
use libimagutil::trace::trace_error;
use libimagcounter::counter::Counter;

pub fn list(rt: &Runtime) {
    rt.cli()
        .subcommand_matches("list")
        .map(|_| {
            debug!("Found 'list' subcommand...");

            Counter::all_counters(rt.store()).map(|iterator| {
                for counter in iterator {
                    counter.map(|c| {
                        let name    = c.name();
                        let value   = c.value();

                        if name.is_err() {
                            trace_error(&name.err().unwrap());
                        } else {

                            if value.is_err() {
                                trace_error(&value.err().unwrap());
                            } else {
                                println!("{} - {}", name.unwrap(), value.unwrap());
                            }
                        }
                    })
                    .map_err(|e| trace_error(&e))
                    .ok();
                }
            })
            .map_err(|e| trace_error(&e))

        });
}
