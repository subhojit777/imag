use std::ops::Drop;

use std::path::PathBuf;
use std::fs::File;
use std::error::Error;

use std::fmt::{Debug, Display, Formatter};
use std::fmt;

use runtime::Runtime;

/**
 * A function which lets the user provide content by editing a temp files which gets removed after
 * the function got the content from it.
 */
pub fn let_user_provide_content(rt: &Runtime) -> Option<String> {
    use std::io::Read;
    use std::fs::File;
    use std::process::Command;
    use std::process::Child;

    let filepath        = "/tmp/imag-tmp.md";
    let file_created    = File::create(filepath)
                                .map(|_| true)
                                .unwrap_or(false);

    if !file_created {
        warn!("Could not create temporary file for user input!");
        return None;
    }

    let output = {
        let mut cmd = Command::new(rt.editor());
        cmd.arg(filepath);
        debug!("cmd = {:?}", cmd);
        cmd.spawn()
           .and_then(|child| {
               child.wait_with_output()
            })
    };

    let process_out = output.map_err(|e| {
        error!("Editor call failed");
        debug!("Editor call failed: {:?}", e);
        return None as Option<String>;
    }).unwrap();

    if !process_out.status.success() {
        error!("Editor call failed");
        debug!("status = {:?}", process_out.status);
        debug!("stdout = {:?}", String::from_utf8(process_out.stdout));
        debug!("stderr = {:?}", String::from_utf8(process_out.stderr));
        return None;
    }

    let mut contents = String::new();
    File::open(filepath).map(|mut file| {
        file.read_to_string(&mut contents);
        Some(contents)
    }).unwrap_or(None)
}
