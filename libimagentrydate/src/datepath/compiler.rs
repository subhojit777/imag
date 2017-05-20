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
