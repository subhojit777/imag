use semver::Version;
use toml::Value;

use libimagstore::store::Entry;

use filters::filter::Filter;

pub struct VersionLt {
    version: Version,
}

impl VersionLt {

    pub fn new(version: Version) -> VersionLt {
        VersionLt { version: version }
    }

}

impl Filter for VersionLt {

    fn filter(&self, e: &Entry) -> bool {
        e.get_header()
            .read("imag.version")
            .map(|val| {
                val.map_or(false, |v| {
                    match v {
                        Value::String(s) => {
                            match Version::parse(&s[..]) {
                                Ok(v) => v < self.version,
                                _ => false
                            }
                        },
                        _ => false,
                    }
                })
            })
            .unwrap_or(false)
    }

}


