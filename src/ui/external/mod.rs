use std::fs::File;

pub mod editor;

pub fn get_tempfile(ext: &str) -> Option<(String, File)> {
    use rand::random;

    let randomname = format!("/tmp/imag-{}.{}", random::<u64>(), ext);
    debug!("Attempting to create tempfile at {}", randomname);
    File::create(randomname.clone())
        .map_err(|e| debug!(" Error -> {}", e))
        .ok()
        .map(|f| (randomname, f))
}
