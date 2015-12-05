mod header;
mod commands;

use std::fmt::{Debug, Formatter};
use std::fmt::Result as FMTResult;

use module::{CommandMap, Module, ModuleResult};
use runtime::Runtime;
use self::commands::*;

pub struct BMModule {
    path: Option<String>,
}

const CALLNAMES : &'static [&'static str] = &[ "bm", "bookmark" ];

impl BMModule {

    pub fn new(rt : &Runtime) -> BMModule {
        BMModule {
            path: None
        }
    }

}

impl Module for BMModule {

    fn callnames(&self) -> &'static [&'static str] {
        CALLNAMES
    }

    fn name(&self) -> &'static str{
        "bookmark"
    }

    fn shutdown(&self, rt : &Runtime) -> ModuleResult {
        Ok(())
    }

    fn get_commands(&self, rt: &Runtime) -> CommandMap {
        let mut hm = CommandMap::new();
        hm.insert("add", add_command);
        hm.insert("list", list_command);
        hm.insert("remove", remove_command);
        hm
    }
}

impl Debug for BMModule {

    fn fmt(&self, fmt: &mut Formatter) -> FMTResult {
        write!(fmt, "[Module][BM]");
        Ok(())
    }

}
