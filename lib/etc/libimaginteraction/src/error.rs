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

use std::error::Error;

use libimagerror::into::IntoError;

error_chain! {
    types {
        InteractionError, InteractionErrorKind, ResultExt, Result;
    }

    errors {
        Unknown             {
            description("Unknown Error")
            display("Unknown Error")
        }

        CLIError            {
            description("Error on commandline")
            display("Error on commandline")
        }

        IdMissingError      {
            description("Commandline: ID missing")
            display("Commandline: ID missing")
        }

        StoreIdParsingError {
            description("Error while parsing StoreId")
            display("Error while parsing StoreId")
        }

        IdSelectingError    {
            description("Error while selecting id")
            display("Error while selecting id")
        }

        ConfigError         {
            description("Configuration error")
            display("Configuration error")
        }

        ConfigMissingError  {
            description("Configuration missing")
            display("Configuration missing")
        }

        ConfigTypeError     {
            description("Config Type Error")
            display("Config Type Error")
        }

        NoConfigError       {
            description("No configuration")
            display("No configuration")
        }

        ReadlineHistoryFileCreationError {
            description("Could not create history file for readline")
            display("Could not create history file for readline")
        }

        ReadlineError       {
            description("Readline error")
            display("Readline error")
        }

    }
}

impl IntoError for InteractionErrorKind {
    type Target = InteractionError;

    fn into_error(self) -> Self::Target {
        InteractionError::from_kind(self)
    }

    fn into_error_with_cause(self, _: Box<Error>) -> Self::Target {
        InteractionError::from_kind(self)
    }
}
