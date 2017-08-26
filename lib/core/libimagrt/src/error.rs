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

generate_error_imports!();
use std::io::Error as IOError;

generate_error_types!(RuntimeError, RuntimeErrorKind,
    Instantiate        => "Could not instantiate",
    IOError            => "IO Error",
    IOLogFileOpenError => "IO Error: Could not open logfile",
    ProcessExitFailure => "Process exited with failure",
    ConfigReadError    => "Error while reading the configuration",
    ConfigTypeError    => "Error while reading the configuration: Type Error",
    GlobalLogLevelConfigMissing => "Global config 'imag.logging.level' missing",
    InvalidLogLevelSpec => "Invalid log level specification: Only 'trace', 'debug', 'info', 'warn', 'error' are allowed",
    TomlReadError       => "Error while reading in TOML document",
    TemplateStringRegistrationError => "Error while registering logging template string",
    ConfigMissingLoggingFormatTrace => "Missing config for logging format for trace logging",
    ConfigMissingLoggingFormatDebug => "Missing config for logging format for debug logging",
    ConfigMissingLoggingFormatInfo => "Missing config for logging format for info logging",
    ConfigMissingLoggingFormatWarn => "Missing config for logging format for warn logging",
    ConfigMissingLoggingFormatError => "Missing config for logging format for error logging"
);

impl From<IOError> for RuntimeError {

    fn from(ioe: IOError) -> RuntimeError {
        RuntimeErrorKind::IOError.into_error_with_cause(Box::new(ioe))
    }

}

