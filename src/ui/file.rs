use std::cell::RefCell;
use std::iter::Iterator;
use std::rc::Rc;
use std::ops::Deref;

use storage::file::File;

/**
 * Trait for a printer which can be used to print data from files
 */
pub trait FilePrinter {

    fn new(verbose: bool, debug: bool) -> Self;

    /*
     * Print a single file
     */
    fn print_file(&self, Rc<RefCell<File>>);

    /*
     * Print a list of files
     */
    fn print_files<I: Iterator<Item = Rc<RefCell<File>>>>(&self, files: I) {
        for file in files {
            self.print_file(file);
        }
    }

    fn print_file_custom<F>(&self, file: Rc<RefCell<File>>, f: &F)
        where F: Fn(Rc<RefCell<File>>) -> Vec<String>
    {
        info!("{}", f(file).join(" "));
    }

    fn print_files_custom<F, I>(&self, files: I, f: &F)
        where I: Iterator<Item = Rc<RefCell<File>>>,
              F: Fn(Rc<RefCell<File>>) -> Vec<String>
    {
        for file in files {
            self.print_file_custom(file, f);
        }
    }

}

/**
 * Printer which prints in debug mode if enabled
 */
struct DebugPrinter {
    debug: bool,
}

impl FilePrinter for DebugPrinter {

    fn new(_: bool, debug: bool) -> DebugPrinter {
        DebugPrinter {
            debug: debug,
        }
    }

    fn print_file(&self, f: Rc<RefCell<File>>) {
        if self.debug {
            debug!("[DebugPrinter] ->\n{:?}", f);
        }
    }

    fn print_file_custom<F>(&self, file: Rc<RefCell<File>>, f: &F)
        where F: Fn(Rc<RefCell<File>>) -> Vec<String>
    {
        if self.debug {
            debug!("[DebugPrinter] ->\n{:?}", f(file).join(" "));
        }
    }

}

/**
 * Simple printer, which just uses the info!() macro or debug!() macro if in debug mode.
 */
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

    fn print_file(&self, f: Rc<RefCell<File>>) {
        use ansi_term::Colour::Cyan;

        if self.debug {
            debug!("{:?}", f);
        } else if self.verbose {
            info!("{}", &*f.deref().borrow());
        } else {
            info!("{}: {}", Cyan.paint("[File]"), f.deref().borrow().id());
        }
    }

    fn print_file_custom<F>(&self, file: Rc<RefCell<File>>, f: &F)
        where F: Fn(Rc<RefCell<File>>) -> Vec<String>
    {
        use ansi_term::Colour::Cyan;

        let s = f(file).join(" ");
        if self.debug {
            debug!("{:?}", s);
        } else if self.verbose {
            info!("{}", s);
        } else {
            info!("{}: {}", Cyan.paint("[File]"), s);
        }
    }

}

/**
 * Table printer to print file information in a nice ASCII-table
 */
pub struct TablePrinter {
    sp:         SimplePrinter,
}

impl FilePrinter for TablePrinter {

    fn new(verbose: bool, debug: bool) -> TablePrinter {
        TablePrinter {
            sp:         SimplePrinter::new(verbose, debug),
        }
    }

    fn print_file(&self, f: Rc<RefCell<File>>) {
        self.sp.print_file(f);
    }

    fn print_files<I: Iterator<Item = Rc<RefCell<File>>>>(&self, files: I) {
        use prettytable::Table;
        use prettytable::row::Row;
        use prettytable::cell::Cell;

        let titles = row!["File#", "Owner", "ID"];

        let mut tab = Table::new();
        tab.set_titles(titles);

        let mut i = 0;
        for file in files {
            debug!("Printing file: {:?}", file);
            i += 1;
            let cell_i  = Cell::new(&format!("{}", i)[..]);
            let cell_o  = Cell::new(&format!("{}", file.deref().borrow().owner_name())[..]);

            let id : String = file.deref().borrow().id().clone().into();
            let cell_id = Cell::new(&id[..]);
            let row = Row::new(vec![cell_i, cell_o, cell_id]);
            tab.add_row(row);
        }

        if i != 0 {
            debug!("Printing {} table entries", i);
            tab.printstd();
        } else {
            debug!("Not printing table because there are zero entries");
        }
    }

    fn print_files_custom<F, I>(&self, files: I, f: &F)
        where I: Iterator<Item = Rc<RefCell<File>>>,
              F: Fn(Rc<RefCell<File>>) -> Vec<String>
    {
        use prettytable::Table;
        use prettytable::row::Row;
        use prettytable::cell::Cell;

        let titles = row!["#", "Module", "ID", "..."];

        let mut tab = Table::new();
        tab.set_titles(titles);

        let mut i = 0;
        for file in files {
            debug!("Printing file: {:?}", file);
            i += 1;
            let cell_i  = Cell::new(&format!("{}", i)[..]);
            let cell_o  = Cell::new(&format!("{}", file.deref().borrow().owner_name())[..]);

            let id : String = file.deref().borrow().id().clone().into();
            let cell_id = Cell::new(&id[..]);

            let mut row = Row::new(vec![cell_i, cell_o, cell_id]);

            for cell in f(file).iter() {
                debug!("Adding custom cell: {:?}", cell);
                row.add_cell(Cell::new(&cell[..]))
            }

            tab.add_row(row);
        }

        if i != 0 {
            debug!("Printing {} table entries", i);
            tab.printstd();
        } else {
            debug!("Not printing table because there are zero entries");
        }
    }

}
