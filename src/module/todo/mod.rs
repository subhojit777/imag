use runtime::Runtime;
use module::Module;
use std::path::Path;
use std::error::Error;

pub struct TodoModule {
    path: Option<String>,
}

impl Module for TodoModule {

    fn new(rt : &Runtime) -> TodoModule {
        TodoModule {
            path: None
        }
    }

    fn name(&self) -> String {
        "Todo".to_string()
    }

    fn execute(&self, rt : &Runtime) -> Option<Error> {
        ( )
    }
}
