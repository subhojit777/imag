use std::ops::DerefMut;
use std::path::PathBuf;

use libimagrt::runtime::Runtime;
use libimagerror::trace::trace_error_exit;
use libimagstore::storeid::StoreId;

use util::build_toml_header;

pub fn update(rt: &Runtime) {
    rt.cli()
        .subcommand_matches("update")
        .map(|scmd| {
            scmd.value_of("id")
                .map(|id| {
                    let path = PathBuf::from(id);
                    let path = match StoreId::new(Some(rt.store().path().clone()), path) {
                        Err(e) => trace_error_exit(&e, 1),
                        Ok(p) => p,
                    };

                    rt.store()
                        .retrieve(path)
                        .map(|mut locked_e| {
                            let mut e = locked_e.deref_mut();

                            scmd.value_of("content")
                                .map(|new_content| {
                                    *e.get_content_mut() = String::from(new_content);
                                    debug!("New content set");
                                });

                            *e.get_header_mut() = build_toml_header(scmd, e.get_header().clone());
                            debug!("New header set");
                        })
                })
        });

}

