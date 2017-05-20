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

/// Error module for the DatePathCompiler type
generate_error_module! {
    generate_error_types!(DatePathCompilerError, DatePathCompilerErrorKind,
        DatePathCompilerError => "Unknown DatePathCompiler error"
    );
}
pub use self::error::DatePathCompilerError;
pub use self::error::DatePathCompilerErrorKind;
pub use self::error::MapErrInto;

/// Result type for this module.
pub mod result {
    use super::error::DatePathCompilerError as DPCE;
    use std::result::Result as RResult;

    pub type Result<T> = RResult<T, DPCE>;
}
use result::Result;

use chrono::naive::datetime::NaiveDateTime;

use libimagstore::storeid::StoreId;

/// A builder for the DatePath object which can then be used to compile a time definition into a
/// StoreId.
#[derive(Builder, Debug)]
#[builder(setter(prefix = "with"))]
pub struct DatePathCompilerBuilder {

    /// The accuracy which should be used to compile the time definition.
    ///
    /// For example a `Accuracy::Hour` will ignore the minute and second from the time definition,
    /// a `Accuracy::Month` will ignore days, hours, minutes and seconds.
    #[builder(default)]
    accuracy : Accuracy,

    /// The formatter which shall be used to compile the time specification.
    #[builder(default)]
    format   : Format,

}

/// The accuracy with which the compiler should compile the time specification
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Accuracy {
    Year,
    Month,
    Day,
    Hour,
    Minute,
    Second
}

impl Default for Accuracy {
    fn default() -> Accuracy {
        Accuracy::Second
    }
}

/// The format which should be used to compile the datetime spec into a StoreId object.
///
/// # Warning
///
/// These settings depend on the Accuracy settings of the compiler as well.
///
/// If the compiler settings only specify an accuracy of `Accuracy::Month`, a setting of
/// `Format::ElementIsFolder` will result in the `month` beeing the file name.
///
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Format {
    /// Each element of the Path is one folder level.
    ///
    /// This is the default.
    ///
    /// # Example
    ///
    /// The date "1st May of 2017, 14:15:16" will be compiled to `2017/05/01/14/15/16`.
    ///
    /// The second is the filename, then.
    ///
    /// # Usecase
    ///
    /// When expecting a lot of entries, this makes sure that the tree is fast-traversible and has
    /// few files per folder (maximum 60).
    ///
    ElementIsFolder,

    /// Each element from Year to Day is folder, below is filename.
    ///
    /// # Example
    ///
    /// The date "1st May of 2017, 14:15:16" will be compiled to `2017/05/01/14-15-16`.
    ///
    /// # Usecase
    ///
    /// When expecting few entries per day.
    ///
    DaysAreFolder,

    /// Each element from Year to Month is folder, below is filename.
    ///
    /// # Example
    ///
    /// The date "1st May of 2017, 14:15:16" will be compiled to `2017/05/01T14-15-16`.
    ///
    /// # Usecase
    ///
    /// When expecting few entries per month.
    ///
    MonthIsFolder,

    /// Each element from Year to Month is folder, below is filename.
    ///
    /// # Example
    ///
    /// The date "1st May of 2017, 14:15:16" will be compiled to `2017/05-01T14-15-16`.
    ///
    /// # Usecase
    ///
    /// When expecting few entries per year.
    /// Might be never used.
    ///
    YearIsFolder,

}

impl Default for Format {
    fn default() -> Format {
        Format::ElementIsFolder
    }
}

pub struct DatePathCompiler {
    accuracy : Accuracy,
    format   : Format,
}

impl DatePathCompiler {

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
    fn compile(&self, datetime: &NaiveDateTime) -> Result<StoreId> {
        unimplemented!()
    }

}

//
// Extension Trait for NaiveDateTime
//

pub trait ToStoreId {
    fn to_store_id(&self, compiler: &DatePathCompiler) -> Result<StoreId>;
}

impl ToStoreId for NaiveDateTime {
    fn to_store_id(&self, compiler: &DatePathCompiler) -> Result<StoreId> {
        compiler.compile(self)
    }
}

