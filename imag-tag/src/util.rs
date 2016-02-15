use std::path::PathBuf;

use semver::Version;

use libimagrt::runtime::Runtime;

pub fn build_entry_path(rt: &Runtime, path_elem: &str) -> PathBuf {
    debug!("Checking path element for version");
    {
        let contains_version = {
            path_elem.split("~")
                .last()
                .map(|version| Version::parse(version).is_ok())
                .unwrap_or(false)
        };

        if !contains_version {
            debug!("Version cannot be parsed inside {:?}", path_elem);
            warn!("Path does not contain version. Will panic now!");
            panic!("No version in path");
        }
    }
    debug!("Version checking succeeded");

    debug!("Building path from {:?}", path_elem);
    let mut path = rt.store().path().clone();

    if path_elem.chars().next() == Some('/') {
        path.push(&path_elem[1..path_elem.len()]);
    } else {
        path.push(path_elem);
    }

    path
}

