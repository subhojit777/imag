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

use std::io::Read;
use std::path::PathBuf;

use email::MimeMessage;

use libimagentryref::hasher::Hasher;
use libimagentryref::hasher::DefaultHasher;
use libimagentryref::error::RefErrorKind as REK;
use libimagentryref::error::ResultExt;
use libimagentryref::result::Result as RResult;

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
        use email::Header;

        let mut s = String::new();
        try!(c.read_to_string(&mut s).chain_err(|| REK::UTF8Error).chain_err(|| REK::IOError));

        MimeMessage::parse(&s)
            .chain_err(|| REK::RefHashingError)
            .and_then(|mail| {
                let has_key = |hdr: &Header, exp: &str| hdr.name == exp;

                let subject_filter = |hdr: &Header| has_key(hdr, "Subject");
                let from_filter    = |hdr: &Header| has_key(hdr, "From");
                let to_filter      = |hdr: &Header| has_key(hdr, "To");

                let filter = subject_filter.or(from_filter).or(to_filter);

                let mut v : Vec<String> = vec![];
                for hdr in mail.headers.iter().filter(|item| filter.filter(item)) {
                    let s = try!(hdr
                        .get_value()
                        .chain_err(|| REK::RefHashingError));

                    v.push(s);
                }
                let s : String = v.join("");

                self.defaulthasher.create_hash(pb, &mut s.as_bytes())
            })
    }

}
