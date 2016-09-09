use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::process::Command;
use std::io::Error as IOError;

use tempfile::NamedTempFile;

pub fn edit_in_tmpfile_with_command(mut cmd: Command, s: &mut String) -> Result<bool, IOError> {
    let file      = try!(NamedTempFile::new());
    let file_path = file.path();
    let mut file  = try!(file.reopen());

    try!(file.write_all(&s.clone().into_bytes()[..]));
    try!(file.sync_data());

    cmd.arg(file_path)
        .status()
        .and_then(|status| {
            if status.success() {
                file.sync_data()
                    .and_then(|_| file.seek(SeekFrom::Start(0)))
                    .and_then(|_| {
                        let mut new_s = String::new();
                        let res = file.read_to_string(&mut new_s);
                        *s = new_s;
                        res
                    })
                    .map(|_| true)
            } else {
                Ok(false)
            }
        })
}

