pub use runtime::Runtime;
pub use std::error::Error;
pub use std::fs::Path;

pub use module::todo::TodoModule;

pub trait Module {

    fn new(&rt : Runtime) -> Self;
    fn callnames() -> &'static [str];
    fn name(&self) -> &'static str;

    fn execute(&self, &rt : Runtime) -> Option<Error>;
    fn shutdown(&self, &rt : Runtime) -> Option<Error>;

}

