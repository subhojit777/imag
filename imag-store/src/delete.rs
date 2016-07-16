use libimagstore::storeid::build_entry_path;
use libimagrt::runtime::Runtime;
use libimagerror::trace::trace_error_exit;

pub fn delete(rt: &Runtime) {
    use std::process::exit;

    rt.cli()
        .subcommand_matches("delete")
        .map(|sub| {
            sub.value_of("id")
                .map(|id| {
                    let path = build_entry_path(rt.store(), id);
                    if path.is_err() {
                        trace_error_exit(&path.unwrap_err(), 1);
                    }
                    let path = path.unwrap();
                    debug!("Deleting file at {:?}", id);

                    rt.store()
                        .delete(path)
                        .map_err(|e| {
                           warn!("Error: {:?}", e);
                           exit(1);
                        })
                })
                .or_else(|| {
                    warn!("No ID passed. Will exit now");
                    exit(1);
                })
        })
        .or_else(|| {
            warn!("No subcommand 'delete'. Will exit now");
            exit(1);
        });
}

