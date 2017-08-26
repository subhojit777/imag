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

use std::io::Write;
use std::io::stderr;
use std::collections::BTreeMap;

use configuration::Configuration;
use error::RuntimeErrorKind as EK;
use error::RuntimeError;
use error::MapErrInto;
use runtime::Runtime;

use libimagerror::into::IntoError;

use clap::ArgMatches;
use log::{Log, LogLevel, LogRecord, LogMetadata};
use toml::Value;
use toml_query::read::TomlValueReadExt;

type ModuleName = String;
type Result<T> = ::std::result::Result<T, RuntimeError>;

enum LogDestination {
    Stderr,
    File(::std::fs::File),
}

impl Default for LogDestination {
    fn default() -> LogDestination {
        LogDestination::Stderr
    }
}

struct ModuleSettings {
    enabled: bool,
    level: LogLevel,

    #[allow(unused)]
    destinations: Vec<LogDestination>,
}

/// Logger implementation for `log` crate.
pub struct ImagLogger {
    global_loglevel     : LogLevel,

    #[allow(unused)]
    global_destinations : Vec<LogDestination>,
    // global_format_trace : ,
    // global_format_debug : ,
    // global_format_info  : ,
    // global_format_warn  : ,
    // global_format_error : ,
    module_settings     : BTreeMap<ModuleName, ModuleSettings>,
}

impl ImagLogger {

    /// Create a new ImagLogger object with a certain level
    pub fn new(matches: &ArgMatches, config: Option<&Configuration>) -> Result<ImagLogger> {
        Ok(ImagLogger {
            global_loglevel     : try!(aggregate_global_loglevel(matches, config)),
            global_destinations : try!(aggregate_global_destinations(matches, config)),
            // global_format_trace : try!(aggregate_global_format_trace(matches, config)),
            // global_format_debug : try!(aggregate_global_format_debug(matches, config)),
            // global_format_info  : try!(aggregate_global_format_info(matches, config)),
            // global_format_warn  : try!(aggregate_global_format_warn(matches, config)),
            // global_format_error : try!(aggregate_global_format_error(matches, config)),
            module_settings     : try!(aggregate_module_settings(matches, config)),
        })
    }

    pub fn global_loglevel(&self) -> LogLevel {
        self.global_loglevel
    }

}

impl Log for ImagLogger {

    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= self.global_loglevel
    }

    fn log(&self, record: &LogRecord) {
        let log_level = record.level();
        let log_location = record.location();
        let log_target = record.target();

        self.module_settings
            .get(log_target)
            .map(|module_setting| {
                if module_setting.enabled && module_setting.level >= log_level {
                    write!(stderr(), "[imag][{}]: {}", log_level, record.args()).ok();
                }
            })
            .unwrap_or_else(|| {
                if self.global_loglevel >= log_level {
                    // Yes, we log
                    write!(stderr(), "[imag][{}]: {}", log_level, record.args()).ok();
                }
            });
    }
}

fn match_log_level_str(s: &str) -> Result<LogLevel> {
    match s {
        "trace" => Ok(LogLevel::Trace),
        "debug" => Ok(LogLevel::Debug),
        "info"  => Ok(LogLevel::Info),
        "warn"  => Ok(LogLevel::Warn),
        "error" => Ok(LogLevel::Error),
        _       => return Err(EK::InvalidLogLevelSpec.into_error()),
    }
}

fn aggregate_global_loglevel(matches: &ArgMatches, config: Option<&Configuration>)
    -> Result<LogLevel>
{
    match config {
        Some(cfg) => match cfg
            .read("imag.logging.level")
            .map_err_into(EK::ConfigReadError)
            {
                Ok(Some(&Value::String(ref s))) => match_log_level_str(s),
                Ok(Some(_)) => Err(EK::ConfigTypeError.into_error()),
                Ok(None)    => Err(EK::GlobalLogLevelConfigMissing.into_error()),
                Err(e)      => Err(e)
            },
        None => {
            if matches.is_present(Runtime::arg_debugging_name()) {
                return Ok(LogLevel::Debug)
            }

            matches
                .value_of(Runtime::arg_verbosity_name())
                .map(match_log_level_str)
                .unwrap_or(Ok(LogLevel::Info))
        }
    }
}

fn translate_destination(raw: &str) -> Result<LogDestination> {
    use std::fs::OpenOptions;

    match raw {
        "-" => Ok(LogDestination::Stderr),
        other => {
            OpenOptions::new()
                .append(true)
                .create(true)
                .open(other)
                .map(LogDestination::File)
                .map_err_into(EK::IOLogFileOpenError)
        }
    }
}


fn translate_destinations(raw: &Vec<Value>) -> Result<Vec<LogDestination>> {
    raw.iter()
        .fold(Ok(vec![]), |acc, val| {
            acc.and_then(|mut v| {
                let dest = match *val {
                    Value::String(ref s) => try!(translate_destination(s)),
                    _ => return Err(EK::ConfigTypeError.into_error()),
                };
                v.push(dest);
                Ok(v)
            })
        })
}

fn aggregate_global_destinations(matches: &ArgMatches, config: Option<&Configuration>)
    -> Result<Vec<LogDestination>>
{

    match config {
        Some(cfg) => match cfg
            .read("imag.logging.destinations")
            .map_err_into(EK::ConfigReadError)
            {
                Ok(Some(&Value::Array(ref a))) => translate_destinations(a),
                Ok(Some(_)) => Err(EK::ConfigTypeError.into_error()),
                Ok(None)    => Err(EK::GlobalLogLevelConfigMissing.into_error()),
                Err(e)      => Err(e)
            },
        None => {
            if let Some(values) = matches.value_of(Runtime::arg_logdest_name()) {
                // parse logdest specification from commandline

                values.split(",")
                    .fold(Ok(vec![]), move |acc, dest| {
                        acc.and_then(|mut v| {
                            v.push(try!(translate_destination(dest)));
                            Ok(v)
                        })
                    })
            } else {
                Ok(vec![ LogDestination::default() ])
            }
        }
    }
}

// fn aggregate_global_format_trace(matches: &ArgMatches, config: Option<&Configuration>)
//     ->
// {
//     unimplemented!()
// }
//
// fn aggregate_global_format_debug(matches: &ArgMatches, config: Option<&Configuration>)
//     ->
// {
//     unimplemented!()
// }
//
// fn aggregate_global_format_info(matches: &ArgMatches, config: Option<&Configuration>)
//     ->
// {
//     unimplemented!()
// }
//
// fn aggregate_global_format_warn(matches: &ArgMatches, config: Option<&Configuration>)
//     ->
// {
//     unimplemented!()
// }
//
// fn aggregate_global_format_error(matches: &ArgMatches, config: Option<&Configuration>)
//     ->
// {
//     unimplemented!()
// }

fn aggregate_module_settings(matches: &ArgMatches, config: Option<&Configuration>)
    -> Result<BTreeMap<ModuleName, ModuleSettings>>
{
    match config {
        Some(cfg) => match cfg
            .read("imag.logging.modules")
            .map_err_into(EK::ConfigReadError)
            {
                Ok(Some(&Value::Table(ref t))) => {
                    // translate the module settings from the table `t`
                    let mut settings = BTreeMap::new();

                    for (module_name, v) in t {
                        let destinations = try!(match v.read("destinations") {
                            Ok(Some(&Value::Array(ref a))) => translate_destinations(a),
                            Ok(Some(_)) => Err(EK::ConfigTypeError.into_error()),
                            Ok(None)    => Err(EK::GlobalLogLevelConfigMissing.into_error()),
                            Err(e)      => Err(e).map_err_into(EK::TomlReadError),
                        });

                        let level = try!(match v.read("level") {
                            Ok(Some(&Value::String(ref s))) => match_log_level_str(s),
                            Ok(Some(_)) => Err(EK::ConfigTypeError.into_error()),
                            Ok(None)    => Err(EK::GlobalLogLevelConfigMissing.into_error()),
                            Err(e)      => Err(e).map_err_into(EK::TomlReadError),
                        });

                        let enabled = try!(match v.read("enabled") {
                            Ok(Some(&Value::Boolean(b))) => Ok(b),
                            Ok(Some(_)) => Err(EK::ConfigTypeError.into_error()),
                            Ok(None)    => Err(EK::GlobalLogLevelConfigMissing.into_error()),
                            Err(e)      => Err(e).map_err_into(EK::TomlReadError),
                        });

                        let module_settings = ModuleSettings {
                            enabled: enabled,
                            level: level,
                            destinations: destinations,
                        };

                        // We don't care whether there was a value, we override it.
                        let _ = settings.insert(module_name.to_owned(), module_settings);
                    }

                    Ok(settings)
                },
                Ok(Some(_)) => Err(EK::ConfigTypeError.into_error()),
                Ok(None)    => Err(EK::GlobalLogLevelConfigMissing.into_error()),
                Err(e)      => Err(e),
            },
        None => {
            write!(stderr(), "No Configuration.").ok();
            write!(stderr(), "cannot find module-settings for logging.").ok();
            write!(stderr(), "Will use global defaults").ok();

            Ok(BTreeMap::new())
        }
    }
}

