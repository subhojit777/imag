use runtime::Runtime;

pub struct ImagErrorBase {
    pub shortdesc : String,
    pub longdesc  : String,
}

pub trait ImagError<'a> {
    fn print(&self, rt: &Runtime);
    fn print_long(&self, rt: &Runtime);
    fn print_short(&self, rt: &Runtime);
}

impl<'a> ImagError<'a> for ImagErrorBase {

    fn print(&self, rt: &Runtime) {
        if self.longdesc.is_empty() {
            let s = format!("Error: {}\n\n{}\n\n",
                            self.shortdesc, self.longdesc);
            rt.print(&s)
        } else {
            let s = format!("Error: {}\n", self.shortdesc);
            rt.print(&s)
        }
    }

    fn print_short(&self, rt : &Runtime) {
        rt.print(&self.shortdesc)
    }

    fn print_long(&self, rt : &Runtime) {
        rt.print(&self.longdesc)
    }

}
