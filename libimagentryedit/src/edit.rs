use std::ops::DerefMut;

use libimagerror::into::IntoError;
use libimagrt::runtime::Runtime;
use libimagstore::store::Entry;
use libimagstore::store::FileLockEntry;

use result::Result;
use error::EditErrorKind;
use error::MapErrInto;

pub trait Edit {
    fn edit_content(&mut self, rt: &Runtime) -> Result<()>;
}

impl Edit for String {

    fn edit_content(&mut self, rt: &Runtime) -> Result<()> {
        edit_in_tmpfile(rt, self).map(|_| ())
    }

}

impl Edit for Entry {

    fn edit_content(&mut self, rt: &Runtime) -> Result<()> {
        edit_in_tmpfile(rt, self.get_content_mut())
            .map(|_| ())
    }

}

impl<'a> Edit for FileLockEntry<'a> {

    fn edit_content(&mut self, rt: &Runtime) -> Result<()> {
        self.deref_mut().edit_content(rt)
    }

}

pub fn edit_in_tmpfile(rt: &Runtime, s: &mut String) -> Result<()> {
    use tempfile::NamedTempFile;
    use std::io::Seek;
    use std::io::Read;
    use std::io::SeekFrom;
    use std::io::Write;

    let file      = try!(NamedTempFile::new().map_err_into(EditErrorKind::IOError));
    let file_path = file.path();
    let mut file  = try!(file.reopen().map_err_into(EditErrorKind::IOError));

    try!(file.write_all(&s.clone().into_bytes()[..]).map_err_into(EditErrorKind::IOError));
    try!(file.sync_data().map_err_into(EditErrorKind::IOError));

    if let Some(mut editor) = rt.editor() {
        let exit_status = editor.arg(file_path).status();

        match exit_status.map(|s| s.success()).map_err(Box::new) {
            Ok(true)  => {
                file.sync_data()
                    .and_then(|_| file.seek(SeekFrom::Start(0)))
                    .and_then(|_| {
                        let mut new_s = String::new();
                        let res = file.read_to_string(&mut new_s);
                        *s = new_s;
                        res
                    })
                    .map(|_| ())
                    .map_err_into(EditErrorKind::IOError)
            },
            Ok(false) => Err(EditErrorKind::ProcessExitFailure.into()),
            Err(e)    => Err(EditErrorKind::IOError.into_error_with_cause(e)),
        }
    } else {
        Err(EditErrorKind::InstantiateError.into())
    }
}
