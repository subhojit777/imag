//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015, 2016 Matthias Beyer <mail@beyermatthias.de> and contributors
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
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;

use libimagstore::store::Store;
use libimagref::reference::Ref;
use libimagref::flags::RefFlags;

use email::MimeMessage;
use email::results::ParsingResult as EmailParsingResult;

use hasher::MailHasher;
use result::Result;
use error::{MapErrInto, MailErrorKind as MEK};

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

pub struct Mail<'a>(Ref<'a>, Buffer);

impl<'a> Mail<'a> {

    /// Imports a mail from the Path passed
    pub fn import_from_path<P: AsRef<Path>>(store: &Store, p: P) -> Result<Mail> {
        let h = MailHasher::new();
        let f = RefFlags::default().with_content_hashing(true).with_permission_tracking(false);
        let p = PathBuf::from(p.as_ref());

        Ref::create_with_hasher(store, p, f, h)
            .map_err_into(MEK::RefCreationError)
            .and_then(|reference| {
                reference.fs_file()
                    .map_err_into(MEK::RefHandlingError)
                    .and_then(|path| File::open(path).map_err_into(MEK::IOError))
                    .and_then(|mut file| {
                        let mut s = String::new();
                        file.read_to_string(&mut s)
                            .map(|_| s)
                            .map_err_into(MEK::IOError)
                    })
                    .map(Buffer::from)
                    .map(|buffer| Mail(reference, buffer))
            })
    }

    /// Opens a mail by the passed hash
    pub fn open<S: AsRef<str>>(store: &Store, hash: S) -> Result<Option<Mail>> {
        Ref::get_by_hash(store, String::from(hash.as_ref()))
            .map_err_into(MEK::FetchByHashError)
            .map_err_into(MEK::FetchError)
            .and_then(|o| match o {
                Some(r) => Mail::from_ref(r).map(Some),
                None => Ok(None),
            })

    }

    /// Implement me as TryFrom as soon as it is stable
    pub fn from_ref(r: Ref<'a>) -> Result<Mail> {
        r.fs_file()
            .map_err_into(MEK::RefHandlingError)
            .and_then(|path| File::open(path).map_err_into(MEK::IOError))
            .and_then(|mut file| {
                let mut s = String::new();
                file.read_to_string(&mut s)
                    .map(|_| s)
                    .map_err_into(MEK::IOError)
            })
            .map(Buffer::from)
            .map(|buffer| Mail(r, buffer))
    }

    pub fn get_field(&self, field: &str) -> Result<Option<String>> {
        self.1
            .parsed()
            .map_err_into(MEK::MailParsingError)
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
