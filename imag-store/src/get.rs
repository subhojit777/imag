use std::process::exit;

use libimagstore::storeid::build_entry_path;
use libimagrt::runtime::Runtime;
use libimagerror::trace::{trace_error, trace_error_exit};

use retrieve::print_entry;

pub fn get(rt: &Runtime) {
    rt.cli()
        .subcommand_matches("get")
        .map(|scmd| {
            scmd.value_of("id")
                .map(|id| {
                    let path = build_entry_path(rt.store(), id);
                    if path.is_err() {
                        trace_error_exit(&path.unwrap_err(), 1);
                    }
                    let path = path.unwrap();
                    debug!("path = {:?}", path);

                    match rt.store().get(path) {
                        Ok(Some(entry)) => print_entry(rt, scmd, entry),
                        Ok(None)        => info!("No entry found"),
                        Err(e)          => trace_error(&e),
                    }
                })
        });
}

