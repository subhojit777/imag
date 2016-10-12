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

use std::default::Default;

generate_error_imports!();

generate_custom_error_types!(HookError, HookErrorKind, CustomData,
    HookExecutionError  => "Hook exec error",
    AccessTypeViolation => "Hook access type violation",
    MutableHooksNotAllowed => "Mutable Hooks are denied"
);

generate_result_helper!(HookError, HookErrorKind);

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Copy)]
pub struct CustomData {
    aborting: bool,
}

impl CustomData {

    pub fn aborting(mut self, b: bool) -> CustomData {
        self.aborting = b;
        self
    }

}

impl Default for CustomData {

    fn default() -> CustomData {
        CustomData {
            aborting: true
        }
    }

}

impl HookError {

    pub fn is_aborting(&self) -> bool {
        match self.custom_data {
            Some(b) => b.aborting,
            None => true
        }
    }

}
