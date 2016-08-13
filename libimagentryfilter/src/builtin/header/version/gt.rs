use semver::Version;
use toml::Value;

use libimagstore::store::Entry;

use filters::filter::Filter;

pub struct VersionGt {
    version: Version,
}

impl VersionGt {

    pub fn new(version: Version) -> VersionGt {
        VersionGt { version: version }
    }

}

impl Filter for VersionGt {

    fn filter(&self, e: &Entry) -> bool {
        e.get_header()
            .read("imag.version")
            .map(|val| {
                val.map_or(false, |v| {
                    match v {
                        Value::String(s) => {
                            match Version::parse(&s[..]) {
                                Ok(v) => v > self.version,
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



