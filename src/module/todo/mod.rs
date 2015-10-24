use runtime::Runtime;
use module::Module;
use module::ModuleResult;
use std::path::Path;
use std::result::Result;

pub struct TodoModule {
    path: Option<String>,
}

const CALLNAMES : &'static [&'static str] = &[ "todo" ];

impl Module for TodoModule {

    fn new(rt : &Runtime) -> TodoModule {
        TodoModule {
            path: None
        }
    }

    fn callnames() -> &'static [&'static str] {
        CALLNAMES
    }

    fn name(&self) -> &'static str{
        "Todo"
    }

    fn execute(&self, rt : &Runtime) -> ModuleResult {
        Ok(())
    }

    fn shutdown(&self, rt : &Runtime) -> ModuleResult {
        Ok(())
    }
}
