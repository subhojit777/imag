use std::io::stdout;

use lister::Lister;
use result::Result;
use error::MapErrInto;

use libimagstore::store::FileLockEntry;
use libimagerror::into::IntoError;

use prettytable::Table;
use prettytable::cell::Cell;
use prettytable::row::Row;

pub struct TableLister<F: Fn(&FileLockEntry) -> Vec<String>> {
    line_generator: F,
    header: Option<Vec<String>>,

    with_idx: bool,
}

impl<F: Fn(&FileLockEntry) -> Vec<String>> TableLister<F> {

    pub fn new(gen: F) -> TableLister<F> {
        TableLister {
            line_generator: gen,
            header: None,
            with_idx: true,
        }
    }

    pub fn with_header(mut self, hdr: Vec<String>) -> TableLister<F> {
        self.header = Some(hdr);
        self
    }

    pub fn with_idx(mut self, b: bool) -> TableLister<F> {
        self.with_idx = b;
        self
    }

}

impl<F: Fn(&FileLockEntry) -> Vec<String>> Lister for TableLister<F> {

    fn list<'b, I: Iterator<Item = FileLockEntry<'b>>>(&self, entries: I) -> Result<()> {
        use error::ListErrorKind as LEK;

        let mut table = Table::new();
        let mut header_len : Option<usize> = None;
        match self.header {
            Some(ref s) => {
                debug!("We have a header... preparing");
                let mut cells : Vec<Cell> = s.iter().map(|s| Cell::new(s)).collect();
                if self.with_idx {
                    cells.insert(0, Cell::new("#"));
                }
                table.set_titles(Row::new(cells));
                header_len = Some(s.len());
            },
            None => {
                debug!("No header for table found... continuing without");
            },
        }

        entries.fold(Ok(table), |table, entry| {
            table.and_then(|mut table| {
                let mut v = (self.line_generator)(&entry);
                {
                    let v_len = v.len();
                    if header_len.is_none() {
                        header_len = Some(v_len);
                    }
                    if header_len.map(|l| v_len > l).unwrap_or(false) {
                        return Err(LEK::FormatError.into_error());
                    }
                    while header_len.map(|l| v.len() != l).unwrap_or(false) {
                        v.push(String::from(""));
                    }
                }

                table.add_row(v.iter().map(|s| Cell::new(s)).collect());
                Ok(table)
            })
        })
        .and_then(|tbl| {
            let mut io = stdout();
            tbl.print(&mut io).map_err_into(LEK::IOError)
        })
    }

}
