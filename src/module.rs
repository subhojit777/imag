use runtime::Runtime;
use error::ImagError;
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

impl<'a> ImagError<'a> for ModuleError {
    fn print(&self, rt: &Runtime) {
        if self.base.longdesc.is_empty() {
            let s = format!("{}: {}\n\n{}\n\n",
                            self.module_name,
                            self.base.shortdesc,
                            self.base.longdesc);
            rt.print(&s)
        } else {
            let s = format!("{}: {}\n",
                            self.module_name,
                            self.base.shortdesc);
            rt.print(&s)
        }
    }

    fn print_short(&self, rt : &Runtime) {
        let s = format!("{}: {}\n", self.module_name, self.base.shortdesc);
        rt.print(&s)
    }

    fn print_long(&self, rt : &Runtime) {
        let s = format!("{}: {}\n\n{}\n\n",
                        self.module_name,
                        self.base.shortdesc,
                        self.base.longdesc);
        rt.print(&s)
    }
}

pub trait Module {

    fn new() -> Self;
    fn load(&self, &rt : Runtime) -> Self;
    fn name(&self) -> String;

    fn execute(&self, &rt : Runtime) -> Option<ModuleError>;

}
