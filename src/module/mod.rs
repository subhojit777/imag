use runtime::Runtime;
use std::error::Error;
use std::fmt::Formatter;
use std::fmt::Result as FMTResult;
use std::fmt::Display;
use std::result::Result;
use std::collections::HashMap;

use storage::backend::{StorageBackend, StorageBackendError};

pub mod bm;

#[derive(Debug)]
pub struct ModuleError {
    desc: String,
}

impl ModuleError {
    pub fn new(desc: &'static str) -> ModuleError {
        ModuleError {
            desc: desc.to_owned().to_string(),
        }
    }
}

impl Error for ModuleError {

    fn description(&self) -> &str {
        &self.desc[..]
    }

    fn cause(&self) -> Option<&Error> {
        None
    }

}

impl Display for ModuleError {
    fn fmt(&self, f: &mut Formatter) -> FMTResult {
        write!(f, "ModuleError: {}", self.description())
    }
}

pub type ModuleResult = Result<(), ModuleError>;
pub type CommandMap<'a> = HashMap<&'a str, fn(&Runtime, &StorageBackend)>;

pub trait Module {

    fn new(rt : &Runtime) -> Self;
    fn callnames() -> &'static [&'static str];
    fn name(&self) -> &'static str;
    fn shutdown(&self, rt : &Runtime) -> ModuleResult;

    fn get_commands(&self, rt: &Runtime) -> CommandMap;

}

