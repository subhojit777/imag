use super::Module;
use std::path::Path;

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
