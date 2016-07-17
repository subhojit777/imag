use std::process::exit;

use libimagrt::runtime::Runtime;

pub fn verify(rt: &Runtime) {
    if rt.store().verify() {
        info!("Store seems to be fine");
    } else {
        warn!("Store seems to be broken somehow");
        exit(1);
    }
}

