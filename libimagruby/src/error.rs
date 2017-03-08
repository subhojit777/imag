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

use ruru::Class;

class!(RImagError);
class!(RImagObjDoesNotExistError);
class!(RImagStoreError);
class!(RImagStoreWriteError);
class!(RImagStoreReadError);
class!(RImagEntryError);
class!(RImagEntryHeaderError);
class!(RImagEntryHeaderReadError);
class!(RImagEntryHeaderWriteError);
class!(RImagTypeError);

pub fn setup() {
    let imag_error = Class::new("RImagError", Some(&Class::from_existing("RuntimeError")));
    Class::new("RImagObjDoesNotExistError"  , Some(&imag_error));

    {
        let imag_store_error = Class::new("RImagStoreError", Some(&imag_error));
        Class::new("RImagStoreWriteError", Some(&imag_store_error));
        Class::new("RImagStoreReadError" , Some(&imag_store_error));
    }

    {
        let imag_entry_error = Class::new("RImagEntryError"             , Some(&imag_error));
        let imag_entry_header_error = Class::new("RImagEntryHeaderError", Some(&imag_entry_error));
        Class::new("RImagEntryHeaderReadError" , Some(&imag_entry_header_error));
        Class::new("RImagEntryHeaderWriteError", Some(&imag_entry_header_error));
    }

    Class::new("RImagTypeError", Some(&imag_error));
}

