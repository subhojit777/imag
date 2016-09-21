use std::result::Result as RResult;
use std::path::Path;

use libimagstore::store::{FileLockEntry, Store};

use mailparse::{MailParseError, ParsedMail, parse_mail};

use result::Result;

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

pub struct Mail<'a> {
    fle: FileLockEntry<'a>,
    buffer: Buffer,
}

impl<'a> Mail<'a> {

    /// Imports a mail from the Path passed
    pub fn import_from_path<P: AsRef<Path>>(store: &Store, p: P) -> Result<Mail> {
        unimplemented!()
    }

    /// Imports a mail from the String passed
    pub fn import_from_string<S: AsRef<str>>(store: &Store, s: S) -> Result<Mail> {
        unimplemented!()
    }

    /// Opens a mail by the passed hash
    pub fn open<S: AsRef<str>>(store: &Store, hash: S) -> Result<Option<Mail>> {
        unimplemented!()
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
