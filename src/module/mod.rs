pub use runtime::Runtime;
pub use std::error::Error;
pub use std::fs::Path;

pub use module::todo::TodoModule;

pub trait Module {

    fn load(&self, &rt : Runtime) -> Option<Self>;
    fn callnames() -> [String];
    fn name(&self) -> String;

    fn execute(&self, &rt : Runtime) -> Option<Error>;
    fn shutdown(&self, &rt : Runtime) -> Option<Error>;

}

pub trait TouchingModule : Module {

    fn load_with_path(&self, rt : &Runtime, path : &Path) -> Self;

}

