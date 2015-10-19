pub use cli::Config;

use std::io::stderr;
use std::io::Write;

pub struct Runtime {
    config : Config,
}

impl Runtime {

    pub fn new(config : Config) -> Runtime {
        Runtime {
            config: config,
        }
    }

    pub fn debug(&self, string : &String) {
        if self.config.is_debugging() {
            println!("{}", string);
        }
    }

    pub fn print(&self, string : &String) {
        if self.config.is_verbose() || self.config.is_debugging() {
            println!("{}", string);
        }
    }

    pub fn trace(string : &String) {
        // writeln!(&mut stderr, "{}", string);
    }

    pub fn is_verbose(&self) -> bool {
        self.config.is_verbose()
    }

    pub fn is_debugging(&self) -> bool {
        self.config.is_debugging()
    }

}
