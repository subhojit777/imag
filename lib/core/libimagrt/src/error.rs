//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015-2018 Matthias Beyer <mail@beyermatthias.de> and contributors
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

error_chain! {
    types {
        RuntimeError, RuntimeErrorKind, ResultExt, Result;
    }

    foreign_links {
        IO(::std::io::Error);
        TomlDeError(::toml::de::Error);
        TomlQueryError(::toml_query::error::Error);
        HandlebarsTemplateError(::handlebars::TemplateError);
    }

    errors {
        Instantiate {
            description("Could not instantiate")
            display("Could not instantiate")
        }

        IOError {
            description("IO Error")
            display("IO Error")
        }

        ProcessExitFailure {
            description("Process exited with failure")
            display("Process exited with failure")
        }

        IOLogFileOpenError {
            description("IO Error: Could not open logfile")
            display("IO Error: Could not open logfile")
        }

        ConfigTypeError(path: String, should_be_type: &'static str) {
            description("Error while reading the configuration: Type Error")
            display("Type Error: '{}' should be '{}'", path, should_be_type)
        }

        GlobalLogLevelConfigMissing {
            description("Global config 'imag.logging.level' missing")
            display("Global config 'imag.logging.level' missing")
        }

        GlobalDestinationConfigMissing {
            description("Global config 'imag.logging.destinations' missing")
            display("Global config 'imag.logging.destinations' missing")
        }

        InvalidLogLevelSpec {
            description("Invalid log level specification: Only 'trace', 'debug', 'info', 'warn', 'error' are allowed")
            display("Invalid log level specification: Only 'trace', 'debug', 'info', 'warn', 'error' are allowed")
        }

        ConfigMissingLoggingFormatTrace {
            description("Missing config for logging format for trace logging")
            display("Missing config for logging format for trace logging")
        }

        ConfigMissingLoggingFormatDebug {
            description("Missing config for logging format for debug logging")
            display("Missing config for logging format for debug logging")
        }

        ConfigMissingLoggingFormatInfo {
            description("Missing config for logging format for info logging")
            display("Missing config for logging format for info logging")
        }

        ConfigMissingLoggingFormatWarn {
            description("Missing config for logging format for warn logging")
            display("Missing config for logging format for warn logging")
        }

        ConfigMissingLoggingFormatError {
            description("Missing config for logging format for error logging")
            display("Missing config for logging format for error logging")
        }

        ConfigTOMLParserError {
            description("Configuration: TOML Parsing error")
            display("Configuration: TOML Parsing error")
        }

        ConfigNoConfigFileFound   {
            description("Configuration: No config file found")
            display("Configuration: No config file found")
        }

        ConfigOverrideError {
            description("Configuration: Config override error")
            display("Configuration: Config override error")
        }

        ConfigOverrideKeyNotAvailable {
            description("Configuration: Key not available")
            display("Configuration: Key not available")
        }

        ConfigOverrideTypeNotMatching {
            description("Configuration: Configuration Type not matching")
            display("Configuration: Configuration Type not matching")
        }

    }
}

