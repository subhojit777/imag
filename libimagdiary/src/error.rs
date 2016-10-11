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

generate_error_module!(
    generate_error_types!(DiaryError, DiaryErrorKind,
        StoreWriteError     => "Error writing store",
        StoreReadError      => "Error reading store",
        CannotFindDiary     => "Cannot find diary",
        CannotCreateNote    => "Cannot create Note object for diary entry",
        DiaryEditError      => "Cannot edit diary entry",
        PathConversionError => "Error while converting paths internally",
        EntryNotInDiary     => "Entry not in Diary",
        IOError             => "IO Error",
        ViewError           => "Error viewing diary entry"
    );
);

pub use self::error::DiaryError;
pub use self::error::DiaryErrorKind;
pub use self::error::MapErrInto;

