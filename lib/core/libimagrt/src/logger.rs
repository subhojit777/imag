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

use std::io::Write;
use std::io::stderr;
use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::ops::Deref;

use error::RuntimeErrorKind as EK;
use error::RuntimeError as RE;
use error::ResultExt;
use runtime::Runtime;

use clap::ArgMatches;
use log::{Log, Level, Record, Metadata};
use toml::Value;
use toml_query::read::TomlValueReadExt;
use toml_query::read::TomlValueReadTypeExt;
use handlebars::Handlebars;

type ModuleName = String;
type Result<T> = ::std::result::Result<T, RE>;

enum LogDestination {
    Stderr,
    File(Arc<Mutex<::std::fs::File>>),
}

impl Default for LogDestination {
    fn default() -> LogDestination {
        LogDestination::Stderr
    }
}

struct ModuleSettings {
    enabled:        bool,
    level:          Option<Level>,

    #[allow(unused)]
    destinations:   Option<Vec<LogDestination>>,
}

/// Logger implementation for `log` crate.
pub struct ImagLogger {
    global_loglevel     : Level,

    #[allow(unused)]
    global_destinations : Vec<LogDestination>,
    // global_format_trace : ,
    // global_format_debug : ,
    // global_format_info  : ,
    // global_format_warn  : ,
    // global_format_error : ,
    module_settings     : BTreeMap<ModuleName, ModuleSettings>,

    handlebars: Handlebars,
}

impl ImagLogger {

    /// Create a new ImagLogger object with a certain level
    pub fn new(matches: &ArgMatches, config: Option<&Value>) -> Result<ImagLogger> {
        let mut handlebars = Handlebars::new();

        handlebars.register_escape_fn(::handlebars::no_escape);

        ::libimaginteraction::format::register_all_color_helpers(&mut handlebars);
        ::libimaginteraction::format::register_all_format_helpers(&mut handlebars);

        {
            let fmt = aggregate_global_format_trace(config)?;
            handlebars.register_template_string("TRACE", fmt)?; // name must be uppercase
        }
        {
            let fmt = aggregate_global_format_debug(config)?;
            handlebars.register_template_string("DEBUG", fmt)?; // name must be uppercase
        }
        {
            let fmt = aggregate_global_format_info(config)?;
            handlebars.register_template_string("INFO", fmt)?; // name must be uppercase
        }
        {
            let fmt = aggregate_global_format_warn(config)?;
            handlebars.register_template_string("WARN", fmt)?; // name must be uppercase
        }
        {
            let fmt = aggregate_global_format_error(config)?;
            handlebars.register_template_string("ERROR", fmt)?; // name must be uppercase
        }

        Ok(ImagLogger {
            global_loglevel     : aggregate_global_loglevel(matches, config)?,
            global_destinations : aggregate_global_destinations(matches, config)?,
            module_settings     : aggregate_module_settings(matches, config)?,
            handlebars          : handlebars,
        })
    }

    pub fn global_loglevel(&self) -> Level {
        self.global_loglevel
    }

}

impl Log for ImagLogger {

    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.global_loglevel
    }

    fn flush(&self) {
        // nothing?
    }

    fn log(&self, record: &Record) {
        if record.module_path().map(|m| m.starts_with("handlebars")).unwrap_or(false) {
            // This is a ugly, yet necessary hack. When logging, we use handlebars for templating.
            // But as the handlebars library itselfs logs via a normal logging macro ("debug!()"),
            // we have a recursion in our chain.
            //
            // To prevent this recursion, we return here.
            //
            // (As of handlebars 0.29.0 - please check whether you can update handlebars if you see
            // this. Hopefully the next version has a compiletime flag to disable logging)
            return;
        }

        let mut data = BTreeMap::new();

        {
            data.insert("level",        format!("{}", record.level()));
            data.insert("module_path",  String::from(record.module_path().unwrap_or("<modulepath unknown>")));
            data.insert("file",         String::from(record.file().unwrap_or("<file unknown>")));
            data.insert("line",         format!("{}", record.line().unwrap_or(0)));
            data.insert("target",       String::from(record.target()));
            data.insert("message",      format!("{}", record.args()));
        }

        let logtext = self
            .handlebars
            .render(&format!("{}", record.level()), &data)
            .unwrap_or_else(|e| format!("Failed rendering logging data: {:?}\n", e));

        let log_to_destination = |d: &LogDestination| match d {
            &LogDestination::Stderr => {
                let _ = write!(stderr(), "{}\n", logtext);
            },
            &LogDestination::File(ref arc_mutex_logdest) => {
                // if there is an error in the lock, we cannot do anything. So we ignore it here.
                let _ = arc_mutex_logdest
                    .deref()
                    .lock()
                    .map(|mut logdest| {
                        write!(logdest, "{}\n", logtext)
                    });
            }
        };

        // hack to get the right target configuration.
        // If there is no element here, we use the empty string which automatically drops through
        // to the unwrap_or_else() case
        let record_target = record
            .target()
            .split("::")
            .next()
            .unwrap_or("");

        self.module_settings
            .get(record_target)
            .map(|module_setting| {
                let set = module_setting.enabled &&
                    module_setting.level.unwrap_or(self.global_loglevel) >= record.level();

                if set {
                    module_setting.destinations.as_ref().map(|destinations| for d in destinations {
                        // If there's an error, we cannot do anything, can we?
                        let _ = log_to_destination(&d);
                    });

                    for d in self.global_destinations.iter() {
                        // If there's an error, we cannot do anything, can we?
                        let _ = log_to_destination(&d);
                    }
                }
            })
        .unwrap_or_else(|| {
            if self.global_loglevel >= record.level() {
                // Yes, we log
                for d in self.global_destinations.iter() {
                    // If there's an error, we cannot do anything, can we?
                    let _ = log_to_destination(&d);
                }
            }
        });
    }
}

fn match_log_level_str(s: &str) -> Result<Level> {
    match s {
        "trace" => Ok(Level::Trace),
        "debug" => Ok(Level::Debug),
        "info"  => Ok(Level::Info),
        "warn"  => Ok(Level::Warn),
        "error" => Ok(Level::Error),
        _       => return Err(RE::from_kind(EK::InvalidLogLevelSpec)),
    }
}

fn aggregate_global_loglevel(matches: &ArgMatches, config: Option<&Value>) -> Result<Level>
{
    fn get_arg_loglevel(matches: &ArgMatches) -> Result<Option<Level>> {
        if matches.is_present(Runtime::arg_debugging_name()) {
            return Ok(Some(Level::Debug))
        }

        match matches.value_of(Runtime::arg_verbosity_name()) {
            Some(v) => match_log_level_str(v).map(Some),
            None    => if matches.is_present(Runtime::arg_verbosity_name()) {
                Ok(Some(Level::Info))
            } else {
                Ok(None)
            },
        }
    }

    if let Some(cfg) = config {
        let cfg_loglevel = cfg
            .read_string("imag.logging.level")?
            .ok_or(RE::from_kind(EK::GlobalLogLevelConfigMissing))
            .and_then(|s| match_log_level_str(&s))?;

        if let Some(cli_loglevel) = get_arg_loglevel(matches)? {
            if cli_loglevel > cfg_loglevel {
                return Ok(cli_loglevel)
            }
        }

        Ok(cfg_loglevel)

    } else {
        get_arg_loglevel(matches).map(|o| o.unwrap_or(Level::Info))
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
                .map(Mutex::new)
                .map(Arc::new)
                .map(LogDestination::File)
                .chain_err(|| EK::IOLogFileOpenError)
        }
    }
}


fn translate_destinations(raw: &Vec<Value>) -> Result<Vec<LogDestination>> {
    raw.iter()
        .fold(Ok(vec![]), |acc, val| {
            acc.and_then(|mut v| {
                let dest = val.as_str()
                    .ok_or_else(|| {
                        let path = "imag.logging.modules.<mod>.destinations".to_owned();
                        let ty   = "Array<String>";
                        RE::from_kind(EK::ConfigTypeError(path, ty))
                    })
                    .and_then(translate_destination)?;
                v.push(dest);
                Ok(v)
            })
        })
}

fn aggregate_global_destinations(matches: &ArgMatches, config: Option<&Value>)
    -> Result<Vec<LogDestination>>
{

    match config {
        Some(cfg) => cfg
            .read("imag.logging.destinations")?
            .ok_or_else(|| RE::from_kind(EK::GlobalDestinationConfigMissing))?
            .as_array()
            .ok_or_else(|| {
                let path = "imag.logging.destinations".to_owned();
                let ty   = "Array";
                RE::from_kind(EK::ConfigTypeError(path, ty))
            })
            .and_then(translate_destinations),
        None => {
            if let Some(values) = matches.value_of(Runtime::arg_logdest_name()) {
                // parse logdest specification from commandline

                values.split(",")
                    .fold(Ok(vec![]), move |acc, dest| {
                        acc.and_then(|mut v| {
                            v.push(translate_destination(dest)?);
                            Ok(v)
                        })
                    })
            } else {
                Ok(vec![ LogDestination::default() ])
            }
        }
    }
}

macro_rules! aggregate_global_format {
    ($read_str:expr, $error_kind_if_missing:expr, $config:expr) => {
        try!($config.ok_or(RE::from_kind($error_kind_if_missing)))
            .read_string($read_str)?
            .ok_or_else(|| RE::from_kind($error_kind_if_missing))
    };
}

fn aggregate_global_format_trace(config: Option<&Value>)
    -> Result<String>
{
    aggregate_global_format!("imag.logging.format.trace",
                            EK::ConfigMissingLoggingFormatTrace,
                            config)
}

fn aggregate_global_format_debug(config: Option<&Value>)
    -> Result<String>
{
    aggregate_global_format!("imag.logging.format.debug",
                            EK::ConfigMissingLoggingFormatDebug,
                            config)
}

fn aggregate_global_format_info(config: Option<&Value>)
    -> Result<String>
{
    aggregate_global_format!("imag.logging.format.info",
                            EK::ConfigMissingLoggingFormatInfo,
                            config)
}

fn aggregate_global_format_warn(config: Option<&Value>)
    -> Result<String>
{
    aggregate_global_format!("imag.logging.format.warn",
                            EK::ConfigMissingLoggingFormatWarn,
                            config)
}

fn aggregate_global_format_error(config: Option<&Value>)
    -> Result<String>
{
    aggregate_global_format!("imag.logging.format.error",
                            EK::ConfigMissingLoggingFormatError,
                            config)
}

fn aggregate_module_settings(_matches: &ArgMatches, config: Option<&Value>)
    -> Result<BTreeMap<ModuleName, ModuleSettings>>
{
    // Helper macro to return the error from Some(Err(_)) and map everything else to an
    // Option<_>
    macro_rules! inner_try {
        ($v:expr) => {
            match $v {
                Some(Ok(v))  => Some(v),
                Some(Err(e)) => return Err(e),
                None         => None,
            }
        }
    };

    match config {
        Some(cfg) => match cfg.read("imag.logging.modules") {
            Ok(Some(&Value::Table(ref t))) => {
                // translate the module settings from the table `t`
                let mut settings = BTreeMap::new();

                for (module_name, v) in t {
                    let destinations = inner_try! {
                        v.read("destinations")?
                            .map(|val| {
                                val.as_array()
                                    .ok_or_else(|| {
                                        let path = "imag.logging.modules.<mod>.destinations".to_owned();
                                        let ty = "Array";
                                        RE::from_kind(EK::ConfigTypeError(path, ty))
                                    })
                                    .and_then(translate_destinations)
                            })
                    };

                    let level = inner_try! {
                        v.read_string("level")?.map(|s| match_log_level_str(&s))
                    };

                    let enabled = v.read("enabled")?
                        .map(|v| v.as_bool().unwrap_or(false))
                        .ok_or_else(|| {
                            let path = "imag.logging.modules.<mod>.enabled".to_owned();
                            let ty = "Boolean";
                            RE::from_kind(EK::ConfigTypeError(path, ty))
                        })?;

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
            Ok(Some(_)) => {
                let path = "imag.logging.modules".to_owned();
                let ty = "Table";
                Err(RE::from_kind(EK::ConfigTypeError(path, ty)))
            },
            Ok(None)    => {
                // No modules configured. This is okay!
                Ok(BTreeMap::new())
            },
            Err(e) => Err(e).map_err(From::from),
        },
        None => {
            write!(stderr(), "No Configuration.").ok();
            write!(stderr(), "cannot find module-settings for logging.").ok();
            write!(stderr(), "Will use global defaults").ok();

            Ok(BTreeMap::new())
        }
    }
}

