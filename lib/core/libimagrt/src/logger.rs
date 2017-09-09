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
use error::RuntimeError as RE;
use error::ResultExt;
use runtime::Runtime;

use clap::ArgMatches;
use log::{Log, LogLevel, LogRecord, LogMetadata};
use toml::Value;
use toml_query::read::TomlValueReadExt;
use handlebars::Handlebars;

type ModuleName = String;
type Result<T> = ::std::result::Result<T, RE>;

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

        handlebars.register_helper("underline"     , Box::new(self::template_helpers::UnderlineHelper));
        handlebars.register_helper("bold"          , Box::new(self::template_helpers::BoldHelper));
        handlebars.register_helper("blink"         , Box::new(self::template_helpers::BlinkHelper));
        handlebars.register_helper("strikethrough" , Box::new(self::template_helpers::StrikethroughHelper));

        {
            let fmt = try!(aggregate_global_format_trace(matches, config));
            try!(handlebars.register_template_string("TRACE", fmt)); // name must be uppercase
        }
        {
            let fmt = try!(aggregate_global_format_debug(matches, config));
            try!(handlebars.register_template_string("DEBUG", fmt)); // name must be uppercase
        }
        {
            let fmt = try!(aggregate_global_format_info(matches, config));
            try!(handlebars.register_template_string("INFO", fmt)); // name must be uppercase
        }
        {
            let fmt = try!(aggregate_global_format_warn(matches, config));
            try!(handlebars.register_template_string("WARN", fmt)); // name must be uppercase
        }
        {
            let fmt = try!(aggregate_global_format_error(matches, config));
            try!(handlebars.register_template_string("ERROR", fmt)); // name must be uppercase
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
        if record.location().module_path().starts_with("handlebars") {
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
        _       => return Err(RE::from_kind(EK::InvalidLogLevelSpec)),
    }
}

fn aggregate_global_loglevel(matches: &ArgMatches, config: Option<&Configuration>)
    -> Result<LogLevel>
{
    match config {
        Some(cfg) => match cfg.read("imag.logging.level") {
            Ok(Some(&Value::String(ref s))) => match_log_level_str(s),
            Ok(Some(_)) => Err(RE::from_kind(EK::ConfigTypeError)),
            Ok(None)    => Err(RE::from_kind(EK::GlobalLogLevelConfigMissing)),
            Err(e)      => Err(e).map_err(From::from),
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
                .chain_err(|| EK::IOLogFileOpenError)
        }
    }
}


fn translate_destinations(raw: &Vec<Value>) -> Result<Vec<LogDestination>> {
    raw.iter()
        .fold(Ok(vec![]), |acc, val| {
            acc.and_then(|mut v| {
                let dest = match *val {
                    Value::String(ref s) => try!(translate_destination(s)),
                    _ => return Err(RE::from_kind(EK::ConfigTypeError)),
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
        Some(cfg) => match cfg.read("imag.logging.destinations") {
            Ok(Some(&Value::Array(ref a))) => translate_destinations(a),
            Ok(Some(_)) => Err(RE::from_kind(EK::ConfigTypeError)),
            Ok(None)    => Err(RE::from_kind(EK::GlobalDestinationConfigMissing)),
            Err(e)      => Err(e).map_err(From::from),
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
        Some(cfg) => match cfg.read(read_str) {
            Ok(Some(&Value::String(ref s))) => Ok(s.clone()),
            Ok(Some(_)) => Err(RE::from_kind(EK::ConfigTypeError)),
            Ok(None)    => Err(RE::from_kind(error_kind_if_missing)),
            Err(e)      => Err(e).map_err(From::from),
        },
        None => match matches.value_of(cli_match_name).map(String::from) {
            Some(s) => Ok(s),
            None    => Err(RE::from_kind(error_kind_if_missing))
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
        Some(cfg) => match cfg.read("imag.logging.modules") {
            Ok(Some(&Value::Table(ref t))) => {
                // translate the module settings from the table `t`
                let mut settings = BTreeMap::new();

                for (module_name, v) in t {
                    let destinations = try!(match v.read("destinations") {
                        Ok(Some(&Value::Array(ref a))) => translate_destinations(a).map(Some),
                        Ok(None)    => Ok(None),
                        Ok(Some(_)) => Err(RE::from_kind(EK::ConfigTypeError)),
                        Err(e)      => Err(e).map_err(From::from),
                    });

                    let level = try!(match v.read("level") {
                        Ok(Some(&Value::String(ref s))) => match_log_level_str(s).map(Some),
                        Ok(None)    => Ok(None),
                        Ok(Some(_)) => Err(RE::from_kind(EK::ConfigTypeError)),
                        Err(e)      => Err(e).map_err(From::from),
                    });

                    let enabled = try!(match v.read("enabled") {
                        Ok(Some(&Value::Boolean(b))) => Ok(b),
                        Ok(None)    => Ok(false),
                        Ok(Some(_)) => Err(RE::from_kind(EK::ConfigTypeError)),
                        Err(e)      => Err(e).map_err(From::from),
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
            Ok(Some(_)) => Err(RE::from_kind(EK::ConfigTypeError)),
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

mod template_helpers {
    use handlebars::{Handlebars, HelperDef, JsonRender, RenderError, RenderContext, Helper};
    use ansi_term::Colour;
    use ansi_term::Style;

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
        let p = try!(h.param(0).ok_or(RenderError::new("Too few arguments")));

        try!(write!(rc.writer(), "{}", color.paint(p.value().render())));
        Ok(())
    }

    #[derive(Clone, Copy)]
    pub struct UnderlineHelper;

    impl HelperDef for UnderlineHelper {
        fn call(&self, h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(),
            RenderError> {
                let p = try!(h.param(0).ok_or(RenderError::new("Too few arguments")));
                let s = Style::new().underline();
                try!(write!(rc.writer(), "{}", s.paint(p.value().render())));
                Ok(())
            }
    }

    #[derive(Clone, Copy)]
    pub struct BoldHelper;

    impl HelperDef for BoldHelper {
        fn call(&self, h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(),
            RenderError> {
                let p = try!(h.param(0).ok_or(RenderError::new("Too few arguments")));
                let s = Style::new().bold();
                try!(write!(rc.writer(), "{}", s.paint(p.value().render())));
                Ok(())
            }
    }

    #[derive(Clone, Copy)]
    pub struct BlinkHelper;

    impl HelperDef for BlinkHelper {
        fn call(&self, h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(),
            RenderError> {
                let p = try!(h.param(0).ok_or(RenderError::new("Too few arguments")));
                let s = Style::new().blink();
                try!(write!(rc.writer(), "{}", s.paint(p.value().render())));
                Ok(())
            }
    }

    #[derive(Clone, Copy)]
    pub struct StrikethroughHelper;

    impl HelperDef for StrikethroughHelper {
        fn call(&self, h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(),
            RenderError> {
                let p = try!(h.param(0).ok_or(RenderError::new("Too few arguments")));
                let s = Style::new().strikethrough();
                try!(write!(rc.writer(), "{}", s.paint(p.value().render())));
                Ok(())
            }
    }

}

