use std::path::PathBuf;

use libimagrt::runtime::Runtime;
use libimagerror::trace::MapErrTrace;
use libimagstore::storeid::StoreId;
use libimagutil::warn_exit::warn_exit;
use libimagutil::warn_result::*;

pub fn delete(rt: &Runtime) {
    rt.cli()
        .subcommand_matches("delete")
        .map(|sub| {
            sub.value_of("id")
                .map(|id| {
                    let path = PathBuf::from(id);
                    let path = try!(StoreId::new(Some(rt.store().path().clone()), path)
                                    .map_err_trace_exit(1));
                    debug!("Deleting file at {:?}", id);

                    rt.store()
                        .delete(path)
                        .map_warn_err(|e| format!("Error: {:?}", e))
                        .map_err_trace_exit(1)
                })
                .or_else(|| warn_exit("No ID passed. Will exit now", 1))
        })
        .or_else(|| warn_exit("No subcommand 'delete'. Will exit now", 1));
}

