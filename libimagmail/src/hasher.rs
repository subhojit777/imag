use std::io::Read;
use std::path::PathBuf;

use mailparse::{MailHeader, parse_mail};

use libimagref::hasher::Hasher;
use libimagref::hasher::DefaultHasher;
use libimagref::error::RefErrorKind as REK;
use libimagref::error::MapErrInto;
use libimagref::result::Result as RResult;
use libimagerror::into::IntoError;

use error::MailErrorKind as MEK;

pub struct MailHasher {
    defaulthasher: DefaultHasher,
}

impl MailHasher {

    pub fn new() -> MailHasher {
        MailHasher { defaulthasher: DefaultHasher::new() }
    }

}

impl Hasher for MailHasher {

    fn hash_name(&self) -> &'static str {
        "default_mail_hasher"
    }

    fn create_hash<R: Read>(&mut self, pb: &PathBuf, c: &mut R) -> RResult<String> {
        use filters::filter::Filter;

        let mut s = String::new();
        try!(c.read_to_string(&mut s).map_err_into(REK::UTF8Error).map_err_into(REK::IOError));

        parse_mail(&s.as_bytes())
            .map_err(Box::new)
            .map_err(|e| MEK::MailParsingError.into_error_with_cause(e))
            .map_err_into(REK::RefHashingError)
            .and_then(|mail| {
                let has_key = |hdr: &MailHeader, exp: &str|
                    hdr.get_key().map(|s| s == exp).unwrap_or(false);

                let subject_filter = |hdr: &MailHeader| has_key(hdr, "Subject");
                let from_filter    = |hdr: &MailHeader| has_key(hdr, "From");
                let to_filter      = |hdr: &MailHeader| has_key(hdr, "To");

                let filter = subject_filter.or(from_filter).or(to_filter);

                let s : String = mail.headers
                    .iter()
                    .filter(|item| filter.filter(item))
                    .filter_map(|hdr| hdr.get_value().ok()) // TODO: Do not hide error here
                    .collect::<Vec<String>>()
                    .join("");

                self.defaulthasher.create_hash(pb, &mut s.as_bytes())
            })
    }

}
