use std::fmt::{Debug, Formatter, Error as FmtError};
use std::result::Result as RResult;

use libimagstore::store::Store;
use libimagstore::storeid::StoreIdIterator;

use diaryid::DiaryId;
use diaryid::FromStoreId;
use is_in_diary::IsInDiary;
use entry::Entry as DiaryEntry;
use error::DiaryError as DE;
use error::DiaryErrorKind as DEK;
use result::Result;

/// A iterator for iterating over diary entries
pub struct DiaryEntryIterator<'a> {
    store: &'a Store,
    name: &'a str,
    iter: StoreIdIterator,

    year: Option<i32>,
    month: Option<u32>,
    day: Option<u32>,
}

impl<'a> Debug for DiaryEntryIterator<'a> {

    fn fmt(&self, fmt: &mut Formatter) -> RResult<(), FmtError> {
        write!(fmt, "DiaryEntryIterator<name = {}, year = {:?}, month = {:?}, day = {:?}>",
               self.name, self.year, self.month, self.day)
    }

}

impl<'a> DiaryEntryIterator<'a> {

    pub fn new(diaryname: &'a str, store: &'a Store, iter: StoreIdIterator) -> DiaryEntryIterator<'a> {
        DiaryEntryIterator {
            store: store,
            name: diaryname,
            iter: iter,

            year: None,
            month: None,
            day: None,
        }
    }

    // Filter by year, get all diary entries for this year
    pub fn year(mut self, year: i32) -> DiaryEntryIterator<'a> {
        self.year = Some(year);
        self
    }

    // Filter by month, get all diary entries for this month (every year)
    pub fn month(mut self, month: u32) -> DiaryEntryIterator<'a> {
        self.month = Some(month);
        self
    }

    // Filter by day, get all diary entries for this day (every year, every year)
    pub fn day(mut self, day: u32) -> DiaryEntryIterator<'a> {
        self.day = Some(day);
        self
    }

}

impl<'a> Iterator for DiaryEntryIterator<'a> {
    type Item = Result<DiaryEntry<'a>>;

    fn next(&mut self) -> Option<Result<DiaryEntry<'a>>> {
        loop {
            let next = self.iter.next();
            debug!("Next element: {:?}", next);
            if next.is_none() {
                return None;
            }
            let next = next.unwrap();

            if next.is_in_diary(self.name) {
                debug!("Seems to be in diary: {:?}", next);
                let id = DiaryId::from_storeid(&next);
                if id.is_none() {
                    continue;
                }
                let id = id.unwrap();

                let y = match self.year  { None => true, Some(y) => y == id.year() };
                let m = match self.month { None => true, Some(m) => m == id.month() };
                let d = match self.day   { None => true, Some(d) => d == id.day() };

                if y && m && d {
                    return Some(self
                                .store
                                .retrieve(next)
                                .map(|fle| DiaryEntry::new(fle))
                                .map_err(|e| DE::new(DEK::StoreReadError, Some(Box::new(e))))
                                );
                }
            } else {
                debug!("Not in the requested diary ({}): {:?}", self.name, next);
            }
        }
    }

}

