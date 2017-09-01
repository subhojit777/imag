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
    generate_error_types!(GPSError, GPSErrorKind,
        StoreReadError   => "Store read error",
        StoreWriteError  => "Store write error",

        HeaderWriteError => "Couldn't write Header for annotation",
        HeaderReadError  => "Couldn't read Header of Entry",
        HeaderTypeError  => "Header field has unexpected type",

        TypeError        => "Type Error",
        DegreeMissing    => "'degree' value missing",
        MinutesMissing   => "'minutes' value missing",
        SecondsMissing   => "'seconds' value missing",
        LongitudeMissing => "'longitude' value missing",
        LatitudeMissing  => "'latitude' value missing",

        NumberConversionError => "Cannot convert number to fit into variable"
    );
);

pub use self::error::GPSError;
pub use self::error::GPSErrorKind;
pub use self::error::MapErrInto;

