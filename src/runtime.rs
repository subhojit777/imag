pub use cli::Config;

use std::io::stderr;
use std::io::Write;

pub struct Runtime<'a> {
    pub config : Config<'a>,
}

impl<'a> Runtime<'a> {

    pub fn new(config : Config<'a>) -> Runtime<'a> {
        Runtime {
            config: config,
        }
    }

    pub fn debug(&self, string : &'static str) {
        if self.config.is_debugging() {
            println!("{}", string);
        }
    }

    pub fn print(&self, string : &'static str) {
        if self.config.is_verbose() || self.config.is_debugging() {
            println!("{}", string);
        }
    }

    pub fn trace(string : &'static str) {
        // writeln!(&mut stderr, "{}", string);
    }

    pub fn is_verbose(&self) -> bool {
        self.config.is_verbose()
    }

    pub fn is_debugging(&self) -> bool {
        self.config.is_debugging()
    }

}
