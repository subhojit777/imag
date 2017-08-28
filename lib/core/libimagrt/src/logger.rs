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
use handlebars::Handlebars;

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
    enabled:        bool,
    level:          Option<LogLevel>,

    #[allow(unused)]
    destinations:   Option<Vec<LogDestination>>,
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

    handlebars: Handlebars,
}

impl ImagLogger {

    /// Create a new ImagLogger object with a certain level
    pub fn new(matches: &ArgMatches, config: Option<&Configuration>) -> Result<ImagLogger> {
        let mut handlebars = Handlebars::new();

        handlebars.register_helper("black"  , Box::new(self::template_helpers::ColorizeBlackHelper));
        handlebars.register_helper("blue"   , Box::new(self::template_helpers::ColorizeBlueHelper));
        handlebars.register_helper("cyan"   , Box::new(self::template_helpers::ColorizeCyanHelper));
        handlebars.register_helper("green"  , Box::new(self::template_helpers::ColorizeGreenHelper));
        handlebars.register_helper("purple" , Box::new(self::template_helpers::ColorizePurpleHelper));
        handlebars.register_helper("red"    , Box::new(self::template_helpers::ColorizeRedHelper));
        handlebars.register_helper("white"  , Box::new(self::template_helpers::ColorizeWhiteHelper));
        handlebars.register_helper("yellow" , Box::new(self::template_helpers::ColorizeYellowHelper));

        {
            let fmt = try!(aggregate_global_format_trace(matches, config));
            try!(handlebars.register_template_string("TRACE", fmt) // name must be uppercase
                .map_err_into(EK::TemplateStringRegistrationError));
        }
        {
            let fmt = try!(aggregate_global_format_debug(matches, config));
            try!(handlebars.register_template_string("DEBUG", fmt) // name must be uppercase
                .map_err_into(EK::TemplateStringRegistrationError));
        }
        {
            let fmt = try!(aggregate_global_format_info(matches, config));
            try!(handlebars.register_template_string("INFO", fmt) // name must be uppercase
                .map_err_into(EK::TemplateStringRegistrationError));
        }
        {
            let fmt = try!(aggregate_global_format_warn(matches, config));
            try!(handlebars.register_template_string("WARN", fmt) // name must be uppercase
                .map_err_into(EK::TemplateStringRegistrationError));
        }
        {
            let fmt = try!(aggregate_global_format_error(matches, config));
            try!(handlebars.register_template_string("ERROR", fmt) // name must be uppercase
                .map_err_into(EK::TemplateStringRegistrationError));
        }

        Ok(ImagLogger {
            global_loglevel     : try!(aggregate_global_loglevel(matches, config)),
            global_destinations : try!(aggregate_global_destinations(matches, config)),
            module_settings     : try!(aggregate_module_settings(matches, config)),
            handlebars          : handlebars,
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
        let mut data = BTreeMap::new();

        {
            data.insert("level",        format!("{}", record.level()));
            data.insert("module_path",  String::from(record.location().module_path()));
            data.insert("file",         String::from(record.location().file()));
            data.insert("line",         format!("{}", record.location().line()));
            data.insert("target",       String::from(record.target()));
            data.insert("message",      format!("{}", record.args()));
        }

        let logtext = self
            .handlebars
            .render(&format!("{}", record.level()), &data)
            .unwrap_or_else(|e| format!("Failed rendering logging data: {:?}\n", e));

        self.module_settings
            .get(record.target())
            .map(|module_setting| {
                let set = module_setting.enabled &&
                    module_setting.level.unwrap_or(self.global_loglevel) >= record.level();

                if set {
                    let _ = write!(stderr(), "{}\n", logtext);
                }
            })
            .unwrap_or_else(|| {
                if self.global_loglevel >= record.level() {
                    // Yes, we log
                    let _ = write!(stderr(), "{}\n", logtext);
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
                Ok(None)    => Err(EK::GlobalDestinationConfigMissing.into_error()),
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

#[inline]
fn aggregate_global_format(
        read_str: &str,
        cli_match_name: &str,
        error_kind_if_missing: EK,
        matches: &ArgMatches,
        config: Option<&Configuration>
    )
-> Result<String>
{
    match config {
        Some(cfg) => match cfg
            .read(read_str)
            .map_err_into(EK::ConfigReadError)
            {
                Ok(Some(&Value::String(ref s))) => Ok(s.clone()),
                Ok(Some(_)) => Err(EK::ConfigTypeError.into_error()),
                Ok(None)    => Err(error_kind_if_missing.into_error()),
                Err(e)      => Err(e)
            },
        None => match matches.value_of(cli_match_name).map(String::from) {
            Some(s) => Ok(s),
            None    => Err(error_kind_if_missing.into_error())
        }
    }
}

fn aggregate_global_format_trace(matches: &ArgMatches, config: Option<&Configuration>)
    -> Result<String>
{
    aggregate_global_format("imag.logging.format.trace",
                            Runtime::arg_override_trace_logging_format(),
                            EK::ConfigMissingLoggingFormatTrace,
                            matches,
                            config)
}

fn aggregate_global_format_debug(matches: &ArgMatches, config: Option<&Configuration>)
    -> Result<String>
{
    aggregate_global_format("imag.logging.format.debug",
                            Runtime::arg_override_debug_logging_format(),
                            EK::ConfigMissingLoggingFormatDebug,
                            matches,
                            config)
}

fn aggregate_global_format_info(matches: &ArgMatches, config: Option<&Configuration>)
    -> Result<String>
{
    aggregate_global_format("imag.logging.format.info",
                            Runtime::arg_override_info_logging_format(),
                            EK::ConfigMissingLoggingFormatInfo,
                            matches,
                            config)
}

fn aggregate_global_format_warn(matches: &ArgMatches, config: Option<&Configuration>)
    -> Result<String>
{
    aggregate_global_format("imag.logging.format.warn",
                            Runtime::arg_override_warn_logging_format(),
                            EK::ConfigMissingLoggingFormatWarn,
                            matches,
                            config)
}

fn aggregate_global_format_error(matches: &ArgMatches, config: Option<&Configuration>)
    -> Result<String>
{
    aggregate_global_format("imag.logging.format.error",
                            Runtime::arg_override_error_logging_format(),
                            EK::ConfigMissingLoggingFormatError,
                            matches,
                            config)
}

fn aggregate_module_settings(_matches: &ArgMatches, config: Option<&Configuration>)
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
                            Ok(Some(&Value::Array(ref a))) => translate_destinations(a).map(Some),
                            Ok(None)    => Ok(None),
                            Ok(Some(_)) => Err(EK::ConfigTypeError.into_error()),
                            Err(e)      => Err(e).map_err_into(EK::TomlReadError),
                        });

                        let level = try!(match v.read("level") {
                            Ok(Some(&Value::String(ref s))) => match_log_level_str(s).map(Some),
                            Ok(None)    => Ok(None),
                            Ok(Some(_)) => Err(EK::ConfigTypeError.into_error()),
                            Err(e)      => Err(e).map_err_into(EK::TomlReadError),
                        });

                        let enabled = try!(match v.read("enabled") {
                            Ok(Some(&Value::Boolean(b))) => Ok(b),
                            Ok(None)    => Ok(false),
                            Ok(Some(_)) => Err(EK::ConfigTypeError.into_error()),
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
                Ok(None)    => {
                    // No modules configured. This is okay!
                    Ok(BTreeMap::new())
                },
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

mod template_helpers {
    use handlebars::{Handlebars, HelperDef, RenderError, RenderContext, Helper};
    use ansi_term::Colour;

    #[derive(Clone, Copy)]
    pub struct ColorizeBlackHelper;

    impl HelperDef for ColorizeBlackHelper {
        fn call(&self, h: &Helper, hb: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
            colorize(Colour::Black, h, hb, rc)
        }
    }

    #[derive(Clone, Copy)]
    pub struct ColorizeBlueHelper;

    impl HelperDef for ColorizeBlueHelper {
        fn call(&self, h: &Helper, hb: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
            colorize(Colour::Blue, h, hb, rc)
        }
    }

    #[derive(Clone, Copy)]
    pub struct ColorizeCyanHelper;

    impl HelperDef for ColorizeCyanHelper {
        fn call(&self, h: &Helper, hb: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
            colorize(Colour::Cyan, h, hb, rc)
        }
    }

    #[derive(Clone, Copy)]
    pub struct ColorizeGreenHelper;

    impl HelperDef for ColorizeGreenHelper {
        fn call(&self, h: &Helper, hb: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
            colorize(Colour::Green, h, hb, rc)
        }
    }

    #[derive(Clone, Copy)]
    pub struct ColorizePurpleHelper;

    impl HelperDef for ColorizePurpleHelper {
        fn call(&self, h: &Helper, hb: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
            colorize(Colour::Purple, h, hb, rc)
        }
    }

    #[derive(Clone, Copy)]
    pub struct ColorizeRedHelper;

    impl HelperDef for ColorizeRedHelper {
        fn call(&self, h: &Helper, hb: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
            colorize(Colour::Red, h, hb, rc)
        }
    }

    #[derive(Clone, Copy)]
    pub struct ColorizeWhiteHelper;

    impl HelperDef for ColorizeWhiteHelper {
        fn call(&self, h: &Helper, hb: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
            colorize(Colour::White, h, hb, rc)
        }
    }

    #[derive(Clone, Copy)]
    pub struct ColorizeYellowHelper;

    impl HelperDef for ColorizeYellowHelper {
        fn call(&self, h: &Helper, hb: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
            colorize(Colour::Yellow, h, hb, rc)
        }
    }

    fn colorize(color: Colour, h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
        use handlebars::JsonRender;
        let p = try!(h.param(0).ok_or(RenderError::new("Too few arguments")));

        try!(write!(rc.writer(), "{}", color.paint(p.value().render())));
        Ok(())
    }
}

