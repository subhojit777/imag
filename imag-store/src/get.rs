use std::path::PathBuf;

use libimagrt::runtime::Runtime;
use libimagerror::trace::{trace_error, trace_error_exit};
use libimagstore::storeid::StoreId;

use retrieve::print_entry;

pub fn get(rt: &Runtime) {
    rt.cli()
        .subcommand_matches("get")
        .map(|scmd| {
            scmd.value_of("id")
                .map(|id| {
                    let path = PathBuf::from(id);
                    let path = match StoreId::new(Some(rt.store().path().clone()), path) {
                        Err(e) => trace_error_exit(&e, 1),
                        Ok(p) => p,
                    };
                    debug!("path = {:?}", path);

                    match rt.store().get(path) {
                        Ok(Some(entry)) => print_entry(rt, scmd, entry),
                        Ok(None)        => info!("No entry found"),
                        Err(e)          => trace_error(&e),
                    }
                })
        });
}

