//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015-2018 Matthias Beyer <mail@beyermatthias.de> and contributors
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; version
// 2.1 of the License.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
//

use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::fs::OpenOptions;
use std::result::Result as RResult;

use libimagstore::store::Store;
use libimagstore::storeid::StoreId;
use libimagstore::store::FileLockEntry;
use libimagentryref::reference::Ref;
use libimagentryref::refstore::RefStore;
use libimagentryref::refstore::UniqueRefPathGenerator;

use email::MimeMessage;
use email::results::ParsingResult as EmailParsingResult;

use error::Result;
use error::{ResultExt, MailError as ME, MailErrorKind as MEK};

struct UniqueMailRefGenerator;
impl UniqueRefPathGenerator for UniqueMailRefGenerator {
    type Error = ME;

    /// The collection the `StoreId` should be created for
    fn collection() -> &'static str {
        "mail"
    }

    /// A function which should generate a unique string for a Path
    fn unique_hash<A: AsRef<Path>>(path: A) -> RResult<String, Self::Error> {
        use filters::filter::Filter;
        use email::Header;

        let mut s = String::new();
        let _     = OpenOptions::new()
            .read(true)
            .write(false)
            .create(false)
            .open(path)?
            .read_to_string(&mut s)?;

        MimeMessage::parse(&s)
            .chain_err(|| MEK::RefCreationError)
            .and_then(|mail| {
                let has_key = |hdr: &Header, exp: &str| hdr.name == exp;

                let subject_filter = |hdr: &Header| has_key(hdr, "Subject");
                let from_filter    = |hdr: &Header| has_key(hdr, "From");
                let to_filter      = |hdr: &Header| has_key(hdr, "To");

                let filter = subject_filter.or(from_filter).or(to_filter);

                let mut v : Vec<String> = vec![];
                for hdr in mail.headers.iter().filter(|item| filter.filter(item)) {
                    let s = hdr
                        .get_value()
                        .chain_err(|| MEK::RefCreationError)?;

                    v.push(s);
                }
                let s : String = v.join("");
                Ok(s)
            })
    }

    /// Postprocess the generated `StoreId` object
    fn postprocess_storeid(sid: StoreId) -> RResult<StoreId, Self::Error> {
        Ok(sid)
    }
}

struct Buffer(String);

impl Buffer {
    pub fn parsed(&self) -> EmailParsingResult<MimeMessage> {
        MimeMessage::parse(&self.0)
    }
}

impl From<String> for Buffer {
    fn from(data: String) -> Buffer {
        Buffer(data)
    }
}

pub struct Mail<'a>(FileLockEntry<'a>, Buffer);

impl<'a> Mail<'a> {

    /// Imports a mail from the Path passed
    pub fn import_from_path<P: AsRef<Path>>(store: &Store, p: P) -> Result<Mail> {
        debug!("Importing Mail from path");
        store.retrieve_ref::<UniqueMailRefGenerator, P>(p)
            .and_then(|reference| {
                debug!("Build reference file: {:?}", reference);
                reference.get_path()
                    .chain_err(|| MEK::RefHandlingError)
                    .and_then(|path| File::open(path).chain_err(|| MEK::IOError))
                    .and_then(|mut file| {
                        let mut s = String::new();
                        file.read_to_string(&mut s)
                            .map(|_| s)
                            .chain_err(|| MEK::IOError)
                    })
                    .map(Buffer::from)
                    .map(|buffer| Mail(reference, buffer))
            })
    }

    /// Opens a mail by the passed hash
    pub fn open<S: AsRef<str>>(store: &Store, hash: S) -> Result<Option<Mail>> {
        debug!("Opening Mail by Hash");
        store.get_ref::<UniqueMailRefGenerator, S>(hash)
            .chain_err(|| MEK::FetchByHashError)
            .chain_err(|| MEK::FetchError)
            .and_then(|o| match o {
                Some(r) => Mail::from_fle(r).map(Some),
                None => Ok(None),
            })
    }

    /// Implement me as TryFrom as soon as it is stable
    pub fn from_fle(fle: FileLockEntry<'a>) -> Result<Mail<'a>> {
        fle.get_path()
            .chain_err(|| MEK::RefHandlingError)
            .and_then(|path| File::open(path).chain_err(|| MEK::IOError))
            .and_then(|mut file| {
                let mut s = String::new();
                file.read_to_string(&mut s)
                    .map(|_| s)
                    .chain_err(|| MEK::IOError)
            })
            .map(Buffer::from)
            .map(|buffer| Mail(fle, buffer))
    }

    pub fn get_field(&self, field: &str) -> Result<Option<String>> {
        debug!("Getting field in mail: {:?}", field);
        self.1
            .parsed()
            .chain_err(|| MEK::MailParsingError)
            .map(|parsed| {
                parsed.headers
                    .iter()
                    .filter(|hdr| hdr.name == field)
                    .nth(0)
                    .and_then(|field| field.get_value().ok())
            })
    }

    pub fn get_from(&self) -> Result<Option<String>> {
        self.get_field("From")
    }

    pub fn get_to(&self) -> Result<Option<String>> {
        self.get_field("To")
    }

    pub fn get_subject(&self) -> Result<Option<String>> {
        self.get_field("Subject")
    }

    pub fn get_message_id(&self) -> Result<Option<String>> {
        self.get_field("Message-ID")
    }

    pub fn get_in_reply_to(&self) -> Result<Option<String>> {
        self.get_field("In-Reply-To")
    }

}
