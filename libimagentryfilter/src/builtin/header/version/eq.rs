use semver::Version;
use toml::Value;

use libimagstore::store::Entry;

use filter::Filter;

pub struct VersionEq {
    version: Version,
}

impl VersionEq {

    pub fn new(version: Version) -> VersionEq {
        VersionEq { version: version }
    }

}

impl Filter for VersionEq {

    fn filter(&self, e: &Entry) -> bool {
        e.get_header()
            .read("imag.version")
            .map(|val| {
                val.map(|v| {
                    match v {
                        Value::String(s) => {
                            match Version::parse(&s[..]) {
                                Ok(v) => v == self.version,
                                _ => false
                            }
                        },
                        _ => false,
                    }
                })
                .unwrap_or(false)
            })
            .unwrap_or(false)
    }

}

