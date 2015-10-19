use runtime::Runtime;
use error::ImagErrorBase;

pub struct ModuleError {
    base: ImagErrorBase,
    module_name: String,
}

impl ModuleError {

    pub fn short<T: Module>(module : &T, short : String) -> ModuleError {
        ModuleError::new(module, short, "".to_string())
    }

    pub fn new<T: Module>(module : &T, short : String, long : String) -> ModuleError {
        ModuleError {
            base: ImagErrorBase {
                shortdesc: short,
                longdesc: long,
            },
            module_name: module.name(),
        }
    }

}

pub trait Module {

    fn new() -> Self;
    fn load(&self, &rt : Runtime) -> Self;
    fn name(&self) -> String;

    fn execute(&self, &rt : Runtime) -> Option<ModuleError>;

}
