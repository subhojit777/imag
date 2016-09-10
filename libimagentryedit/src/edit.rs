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
    use libimagutil::edit::edit_in_tmpfile_with_command;

    rt.editor()
        .ok_or(EditErrorKind::NoEditor.into_error())
        .and_then(|editor| {
            edit_in_tmpfile_with_command(editor, s)
                .map_err_into(EditErrorKind::IOError)
                .and_then(|worked| {
                    if !worked {
                        Err(EditErrorKind::ProcessExitFailure.into())
                    } else {
                        Ok(())
                    }
                })
        })
}

