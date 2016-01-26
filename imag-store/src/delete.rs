use std::path::PathBuf;

use libimagrt::runtime::Runtime;

use util::build_entry_path;

pub fn delete(rt: &Runtime) {
    use std::process::exit;

    rt.cli()
        .subcommand_matches("delete")
        .map(|sub| {
            sub.value_of("id")
                .map(|id| {
                    debug!("Deleting file at {:?}", id);
                    rt.store()
                        .delete(build_entry_path(rt, id))
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

