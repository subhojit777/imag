use std::path::PathBuf;

use libimagrt::runtime::Runtime;
use libimagerror::trace::trace_error_exit;
use libimagstore::storeid::StoreId;
use libimagutil::warn_exit::warn_exit;

pub fn delete(rt: &Runtime) {
    use std::process::exit;

    rt.cli()
        .subcommand_matches("delete")
        .map(|sub| {
            sub.value_of("id")
                .map(|id| {
                    let path = PathBuf::from(id);
                    let path = try!(StoreId::new(Some(rt.store().path().clone()), path)
                                    .map_err(|e| trace_error_exit(&e, 1)));
                    debug!("Deleting file at {:?}", id);

                    rt.store()
                        .delete(path)
                        .map_err(|e| {
                           warn!("Error: {:?}", e);
                           exit(1);
                        })
                })
                .or_else(|| warn_exit("No ID passed. Will exit now", 1))
        })
        .or_else(|| warn_exit("No subcommand 'delete'. Will exit now", 1));
}

