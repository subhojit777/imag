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

