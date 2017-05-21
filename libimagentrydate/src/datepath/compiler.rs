//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015, 2016 Matthias Beyer <mail@beyermatthias.de> and contributors
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; version
// 2.1 of the License.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
//

use std::path::PathBuf;

use chrono::naive::datetime::NaiveDateTime;
use chrono::Datelike;
use chrono::Timelike;

use libimagstore::storeid::StoreId;

use datepath::accuracy::Accuracy;
use datepath::format::Format;
use datepath::result::Result;
use datepath::error::DatePathCompilerErrorKind as DPCEK;
use datepath::error::MapErrInto;

pub struct DatePathCompiler {
    accuracy : Accuracy,
    format   : Format,
}

impl DatePathCompiler {

    pub fn new(accuracy: Accuracy, format: Format) -> DatePathCompiler {
        DatePathCompiler {
            accuracy : accuracy,
            format   : format,
        }
    }

    /// Compile a NaiveDateTime object into a StoreId object.
    ///
    /// # More information
    ///
    /// See the documentations of the `Format` and the `Accuracy` types as well.
    ///
    /// # Warnings
    ///
    /// This does _not_ guarantee that the StoreId can be created, is not yet in the store or
    /// anything else. Overall, this is just a `spec->path` compiler which is really stupid.
    ///
    /// # Return value
    ///
    /// The `StoreId` object on success.
    ///
    pub fn compile(&self, module_name: &str, datetime: &NaiveDateTime) -> Result<StoreId> {
        let mut s = format!("{}/", module_name);

        if self.accuracy.has_year_accuracy() /* always true */ {
            s.push_str(&format!("{}", datetime.year()));
        }

        if self.accuracy.has_month_accuracy() {
            match self.format {
                Format::ElementIsFolder
                    | Format::DaysAreFolder
                    | Format::MonthIsFolder
                    | Format::YearIsFolder
                    => s.push_str(&format!("/{}", datetime.month())),
            }
        }

        if self.accuracy.has_day_accuracy() {
            match self.format {
                Format::ElementIsFolder
                    | Format::DaysAreFolder
                    | Format::MonthIsFolder
                    => s.push_str(&format!("/{}", datetime.day())),
                Format::YearIsFolder
                    => s.push_str(&format!("-{}", datetime.day())),
            }
        }

        if self.accuracy.has_hour_accuracy() {
            match self.format {
                Format::ElementIsFolder
                    | Format::DaysAreFolder
                    => s.push_str(&format!("/{}", datetime.hour())),
                Format::YearIsFolder
                    | Format::MonthIsFolder
                    => s.push_str(&format!("-{}", datetime.hour())),
            }
        }

        if self.accuracy.has_minute_accuracy() {
            match self.format {
                Format::ElementIsFolder
                    => s.push_str(&format!("/{}", datetime.minute())),
                Format::YearIsFolder
                    | Format::MonthIsFolder
                    | Format::DaysAreFolder
                    => s.push_str(&format!("-{}", datetime.minute())),
            }
        }

        if self.accuracy.has_second_accuracy() {
            match self.format {
                Format::ElementIsFolder
                    => s.push_str(&format!("/{}", datetime.second())),
                Format::YearIsFolder
                    | Format::MonthIsFolder
                    | Format::DaysAreFolder
                    => s.push_str(&format!("-{}", datetime.second())),
            }
        }

        StoreId::new_baseless(PathBuf::from(s))
            .map_err_into(DPCEK::StoreIdBuildFailed)
    }

}

#[cfg(test)]
mod test {
    use super::*;
    use datepath::accuracy::Accuracy;
    use datepath::format::Format;

    use chrono::naive::date::NaiveDate;
    use chrono::naive::datetime::NaiveDateTime;

    #[test]
    fn test_compiler_compile_simple() {
        let dt       = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        let compiler = DatePathCompiler::new(Accuracy::default(), Format::default());
        let res      = compiler.compile("testmodule", &dt);

        assert!(res.is_ok());
        let res = res.unwrap();

        let s = res.to_str();

        assert!(s.is_ok());
        let s = s.unwrap();

        assert_eq!("testmodule/2000/01/01/00/00/00", s);
    }

    fn test_accuracy(acc: Accuracy, dt: NaiveDateTime, modname: &str, matchstr: &str) {
        let compiler = DatePathCompiler::new(acc, Format::default());
        let res      = compiler.compile(modname, &dt);

        assert!(res.is_ok());
        let res = res.unwrap();

        let s = res.to_str();

        assert!(s.is_ok());
        let s = s.unwrap();

        assert_eq!(matchstr, s);
    }

    #[test]
    fn test_compiler_compile_year_accuracy() {
        let dt = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        test_accuracy(Accuracy::Year, dt, "module", "module/2000");
    }

    #[test]
    fn test_compiler_compile_month_accuracy() {
        let dt = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        test_accuracy(Accuracy::Month, dt, "module", "module/2000/01");
    }

    #[test]
    fn test_compiler_compile_day_accuracy() {
        let dt = NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0);
        test_accuracy(Accuracy::Day, dt, "module", "module/2000/01/01");
    }

    #[test]
    fn test_compiler_compile_year_paddning() {
        let dt = NaiveDate::from_ymd(1, 1, 1).and_hms(0, 0, 0);
        test_accuracy(Accuracy::Day, dt, "module", "module/0001/01/01");
    }

}

