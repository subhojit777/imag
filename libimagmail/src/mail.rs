use std::result::Result as RResult;
use std::path::Path;
use std::path::PathBuf;

use libimagstore::store::{FileLockEntry, Store};
use libimagref::reference::Ref;
use libimagref::flags::RefFlags;

use mailparse::{MailParseError, ParsedMail, parse_mail};

use hasher::MailHasher;
use result::Result;
use error::{MapErrInto, MailErrorKind as MEK};

struct Buffer(String);

impl Buffer {
    pub fn parsed<'a>(&'a self) -> RResult<ParsedMail<'a>, MailParseError> {
        parse_mail(self.0.as_bytes())
    }
}

impl From<String> for Buffer {
    fn from(data: String) -> Buffer {
        Buffer(data)
    }
}

pub struct Mail<'a>(Ref<'a>);

impl<'a> Mail<'a> {

    /// Imports a mail from the Path passed
    pub fn import_from_path<P: AsRef<Path>>(store: &Store, p: P) -> Result<Mail> {
        let h = MailHasher::new();
        let f = RefFlags::default().with_content_hashing(true).with_permission_tracking(false);
        let p = PathBuf::from(p.as_ref());

        Ref::create_with_hasher(store, p, f, h)
            .map_err_into(MEK::RefCreationError)
            .map(|r| Mail(r))
    }

    /// Opens a mail by the passed hash
    pub fn open<S: AsRef<str>>(store: &Store, hash: S) -> Result<Option<Mail>> {
        Ref::get_by_hash(store, String::from(hash.as_ref()))
            .map(|opt| opt.map(|r| Mail(r)))
            .map_err_into(MEK::FetchByHashError)
            .map_err_into(MEK::FetchError)
    }

    pub fn get_field<S: AsRef<str>>(&self, field: S) -> Result<Option<&str>> {
        unimplemented!()
    }

    pub fn get_from(&self) -> Result<Option<&str>> {
        unimplemented!()
    }

    pub fn get_to(&self) -> Result<Option<&str>> {
        unimplemented!()
    }

    pub fn get_subject(&self) -> Result<Option<&str>> {
        unimplemented!()
    }

    pub fn get_message_id(&self) -> Result<Option<&str>> {
        unimplemented!()
    }

    pub fn get_in_reply_to(&self) -> Result<Option<&str>> {
        unimplemented!()
    }

}
