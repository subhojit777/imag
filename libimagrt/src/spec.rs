use std::io::Write;

use clap::{App, ArgMatches, Shell};

/// An abstraction over `clap::App` functionality needed for initializing `Runtime`. Different
/// implementations can be used for testing `imag` binaries without running them as separate
/// processes.
pub trait CliSpec<'a> {
    fn name(&self) -> &str;
    fn matches(self) -> ArgMatches<'a>;
    fn completions<W: Write, S: Into<String>>(&mut self, _: S, _: Shell, _: &mut W) {}
}

impl<'a> CliSpec<'a> for App<'a, 'a> {
    fn name(&self) -> &str {
        self.get_name()
    }

    fn matches(self) -> ArgMatches<'a> {
        self.get_matches()
    }

    fn completions<W: Write, S: Into<String>>(&mut self,
                                              bin_name: S,
                                              for_shell: Shell,
                                              buf: &mut W) {

        self.gen_completions_to(bin_name, for_shell, buf);
    }
}
