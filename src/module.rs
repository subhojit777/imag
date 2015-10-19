use runtime::Runtime;
use std::error::Error;

pub trait Module {

    fn load(self, &rt : Runtime) -> Self;
    fn name(self) -> String;

    fn execute(&self, &rt : Runtime) -> Option<Error>;

}
