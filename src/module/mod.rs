use runtime::Runtime;
use std::error::Error;
use std::path::Path;

use module::todo::TodoModule;

mod todo;

pub trait Module {

    fn new(&rt : Runtime) -> Self;
    fn callnames() -> &'static [str];
    fn name(&self) -> &'static str;

    fn execute(&self, &rt : Runtime) -> Option<Error>;
    fn shutdown(&self, &rt : Runtime) -> Option<Error>;

}

