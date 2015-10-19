use runtime::Runtime;

pub struct ModuleError {
    shortdesc : String,
    longdesc  : String,
}

impl ModuleError {

    pub fn short(short : String) -> ModuleError {
        ModuleError::new(short, "".to_string())
    }

    pub fn new(short : String, long : String) -> ModuleError {
        ModuleError {
            shortdesc: short,
            longdesc: long,
        }
    }

    pub fn print(&self, rt : &Runtime) {
        if self.longdesc.is_empty() {
            let s = format!("Error: {}\n\n{}\n\n",
                            self.shortdesc, self.longdesc);
            rt.print(&s)
        } else {
            let s = format!("Error: {}\n", self.shortdesc);
            rt.print(&s)
        }
    }

    pub fn print_short(&self, rt : &Runtime) {
        rt.print(&self.shortdesc)
    }

    pub fn print_long(&self, rt : &Runtime) {
        rt.print(&self.longdesc)
    }

}

pub trait Module {

    fn load(self, &rt : Runtime) -> Self;
    fn name(self) -> String;

    fn execute(self, &rt : Runtime) -> Option<ModuleError>;

}
