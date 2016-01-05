use runtime::Runtime;

/**
 * A function which lets the user provide content by editing a temp files which gets removed after
 * the function got the content from it.
 */
pub fn let_user_provide_content(rt: &Runtime) -> Option<String> {
    use std::io::Read;
    use std::fs::File;
    use std::process::exit;

    let filepath        = "/tmp/imag-tmp.md";
    let file_created    = File::create(filepath)
                                .map(|_| true)
                                .unwrap_or(false);

    if !file_created {
        warn!("Could not create temporary file for user input!");
        return None;
    }

    let output = {
        let mut cmd = rt.editor();
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
        file.read_to_string(&mut contents)
            .map_err(|e| {
                error!("Error reading content: {}", e);
                debug!("Error reading content: {:?}", e);
                exit(1);
            })
            .is_ok();
        Some(contents)
    }).unwrap_or(None)
}

/**
 * Edit some content in a temporary file. If anything failes within this routine, it returns the
 * old content and false.
 * If the editing succeeded, it returns the new content and true
 */
pub fn edit_content(rt: &Runtime, old_content: String) -> (String, bool) {
    use std::io::Read;
    use std::io::Write;
    use std::fs::File;
    use std::process::exit;

    let filepath = "/tmp/imag-tmp.md";
    {
        let mut file = match File::create(filepath) {
            Ok(f) => f,
            Err(e) => {
                error!("Error creating file {}", filepath);
                debug!("Error creating file at '{}', error = {}", filepath, e);
                exit(1);
            }
        };

        file.write(old_content.as_ref())
            .map_err(|e| {
                error!("Error writing content: {}", e);
                debug!("Error writing content: {:?}", e);
                exit(1);
            }).is_ok();
    }
    debug!("Ready with putting old content into the file");

    let output = {
        let mut cmd = rt.editor();
        cmd.arg(filepath);
        debug!("cmd = {:?}", cmd);
        cmd.spawn()
           .and_then(|child| child.wait_with_output())
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
        return (old_content, false);
    }

    let mut contents = String::new();
    File::open(filepath).map(|mut file| {
        file.read_to_string(&mut contents).map_err(|e| {
            error!("Error reading content: {}", e);
            debug!("Error reading content: {:?}", e);
            exit(1);
        }).is_ok();
        (contents, true)
    }).unwrap_or((old_content, false))
}

