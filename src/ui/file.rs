use std::iter::Iterator;

use runtime::Runtime;
use storage::file::File;

pub trait FilePrinter {

    fn new(verbose: bool, debug: bool) -> Self;

    /*
     * Print a single file
     */
    fn print_file(&self, &File);

    /*
     * Print a list of files
     */
    fn print_files<'a, I: Iterator<Item = File<'a>>>(&self, files: I) {
        for file in files {
            self.print_file(&file);
        }
    }

}

struct DebugPrinter {
    debug: bool,
}

impl FilePrinter for DebugPrinter {

    fn new(verbose: bool, debug: bool) -> DebugPrinter {
        DebugPrinter {
            debug: debug,
        }
    }

    fn print_file(&self, f: &File) {
        if self.debug {
            debug!("[DebugPrinter] ->\n{:?}", f);
        }
    }

}

struct SimplePrinter {
    verbose:    bool,
    debug:      bool,
}

impl FilePrinter for SimplePrinter {

    fn new(verbose: bool, debug: bool) -> SimplePrinter {
        SimplePrinter {
            debug:      debug,
            verbose:    verbose,
        }
    }

    fn print_file(&self, f: &File) {
        if self.debug {
            debug!("{:?}", f);
        } else if self.verbose {
            info!("{}", f);
        } else {
            info!("[File]: {}", f.id());
        }
    }

}

