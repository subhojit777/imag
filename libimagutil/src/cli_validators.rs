//! Functions to be used for clap::Arg::validator()
//! to validate arguments

use std::path::PathBuf;
use boolinator::Boolinator;

pub fn is_file(s: String) -> Result<(), String> {
    PathBuf::from(s.clone()).is_file().as_result((), format!("Not a File: {}", s))
}

pub fn is_directory(s: String) -> Result<(), String> {
    PathBuf::from(s.clone()).is_dir().as_result((), format!("Not a Directory: {}", s))
}

pub fn is_integer(s: String) -> Result<(), String> {
    use std::str::FromStr;

    let i : Result<i64, _> = FromStr::from_str(&s);
    i.map(|_| ()).map_err(|_| format!("Not an integer: {}", s))
}

pub fn is_url(s: String) -> Result<(), String> {
    use url::Url;
    Url::parse(&s).map(|_| ()).map_err(|_| format!("Not a URL: {}", s))
}

