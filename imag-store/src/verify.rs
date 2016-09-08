use libimagrt::runtime::Runtime;
use libimagutil::warn_exit::warn_exit;

pub fn verify(rt: &Runtime) {
    if rt.store().verify() {
        info!("Store seems to be fine");
    } else {
        warn_exit("Store seems to be broken somehow", 1);
    }
}

