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
    generate_error_types!(RefError, RefErrorKind,
        StoreReadError          => "Store read error",
        StoreWriteError         => "Store write error",
        IOError                 => "IO Error",
        UTF8Error               => "UTF8 Error",
        HeaderTypeError         => "Header type error",
        HeaderFieldMissingError => "Header field missing error",
        HeaderFieldWriteError   => "Header field cannot be written",
        HeaderFieldReadError    => "Header field cannot be read",
        HeaderFieldAlreadyExistsError => "Header field already exists, cannot override",
        PathUTF8Error => "Path cannot be converted because of UTF8 Error",
        PathHashingError => "Path cannot be hashed",
        PathCanonicalizationError => "Path cannot be canonicalized",

        TypeConversionError => "Couldn't convert types",
        RefToDisplayError => "Cannot convert Ref to string to show it to user",

        RefNotInStore => "Ref/StoreId does not exist in store",

        RefTargetDoesNotExist       => "Ref Target does not exist",
        RefTargetPermissionError    => "Ref Target permissions insufficient for referencing",
        RefTargetCannotBeHashed     => "Ref Target cannot be hashed (is it a directory?)",
        RefTargetFileCannotBeOpened => "Ref Target File cannot be open()ed",
        RefTargetCannotReadPermissions => "Ref Target: Cannot read permissions"
    );
);

pub use self::error::RefError;
pub use self::error::RefErrorKind;
pub use self::error::MapErrInto;

