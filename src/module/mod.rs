use runtime::Runtime;
use std::error::Error;
use std::fmt::Formatter;
use std::fmt::Result as FMTResult;
use std::fmt::Display;
use std::result::Result;

use storage::backend::StorageBackend;
use self::command::ExecutableCommand;

mod command;
pub mod bm;

#[derive(Debug)]
pub struct ModuleError {
    desc: String,
}

impl ModuleError {
    fn mk(desc: &'static str) -> ModuleError {
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

pub trait Module {

    fn new(rt : &Runtime) -> Self;
    fn callnames() -> &'static [&'static str];
    fn name(&self) -> &'static str;

    fn execute(&self, rt : &Runtime) -> ModuleResult;
    fn shutdown(&self, rt : &Runtime) -> ModuleResult;

    fn getCommandBuilder<T, F>() -> F
        where F: FnOnce(StorageBackend) -> T,
              T: ExecutableCommand;

}

