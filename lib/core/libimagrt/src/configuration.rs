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

use std::path::PathBuf;
use std::ops::Deref;

use toml::Value;
use clap::App;

error_chain! {
    types {
        ConfigError, ConfigErrorKind, ResultExt, Result;
    }

    errors {
        TOMLParserError {
            description("TOML Parsing error")
            display("TOML Parsing error")
        }

        NoConfigFileFound   {
            description("No config file found")
            display("No config file found")
        }

        ConfigOverrideError {
            description("Config override error")
            display("Config override error")
        }

        ConfigOverrideKeyNotAvailable {
            description("Key not available")
            display("Key not available")
        }

        ConfigOverrideTypeNotMatching {
            description("Configuration Type not matching")
            display("Configuration Type not matching")
        }
    }
}
use self::ConfigErrorKind as CEK;
use self::ConfigError as CE;

/// `Configuration` object
///
/// Holds all config variables which are globally available plus the configuration object from the
/// config parser, which can be accessed.
#[derive(Debug)]
pub struct Configuration {

    /// The plain configuration object for direct access if necessary
    config: Value,

    /// The verbosity the program should run with
    verbosity: bool,

    /// The editor which should be used
    editor: Option<String>,

    ///The options the editor should get when opening some file
    editor_opts: String,
}

impl Configuration {

    /// Get a new configuration object.
    ///
    /// The passed runtimepath is used for searching the configuration file, whereas several file
    /// names are tested. If that does not work, the home directory and the XDG basedir are tested
    /// with all variants.
    ///
    /// If that doesn't work either, an error is returned.
    pub fn new(config_searchpath: &PathBuf) -> Result<Configuration> {
        fetch_config(&config_searchpath).map(|cfg| {
            let verbosity   = get_verbosity(&cfg);
            let editor      = get_editor(&cfg);
            let editor_opts = get_editor_opts(&cfg);

            debug!("Building configuration");
            debug!("  - verbosity  : {:?}", verbosity);
            debug!("  - editor     : {:?}", editor);
            debug!("  - editor-opts: {}", editor_opts);

            Configuration {
                config: cfg,
                verbosity: verbosity,
                editor: editor,
                editor_opts: editor_opts,
            }
        })
    }

    /// Get a new configuration object built from the given toml value.
    pub fn with_value(value: Value) -> Configuration {
        Configuration{
            verbosity: get_verbosity(&value),
            editor: get_editor(&value),
            editor_opts: get_editor_opts(&value),
            config: value,
        }
    }

    /// Get the Editor setting from the configuration
    pub fn editor(&self) -> Option<&String> {
        self.editor.as_ref()
    }

    /// Get the underlying configuration TOML object
    pub fn config(&self) -> &Value {
        &self.config
    }

    /// Get the configuration of the store, if any.
    pub fn store_config(&self) -> Option<&Value> {
        match self.config {
            Value::Table(ref tabl) => tabl.get("store"),
            _ => None,
        }
    }

    /// Override the configuration.
    /// The `v` parameter is expected to contain 'key=value' pairs where the key is a path in the
    /// TOML tree, the value to be an appropriate value.
    ///
    /// The override fails if the configuration which is about to be overridden does not exist or
    /// the `value` part cannot be converted to the type of the configuration value.
    ///
    /// If `v` is empty, this is considered to be a successful `override_config()` call.
    pub fn override_config(&mut self, v: Vec<String>) -> Result<()> {
        use libimagutil::key_value_split::*;
        use toml_query::read::TomlValueReadExt;

        let iter = v.into_iter()
            .map(|s| { debug!("Trying to process '{}'", s); s })
            .filter_map(|s| match s.into_kv() {
                Some(kv) => Some(kv.into()),
                None => {
                    warn!("Could split at '=' - will be ignore override");
                    None
                }
            })
            .map(|(k, v)| self
                 .config
                 .read(&k[..])
                 .chain_err(|| CEK::TOMLParserError)
                 .map(|toml| match toml {
                    Some(value) => match into_value(value, v) {
                        Some(v) => {
                            info!("Successfully overridden: {} = {}", k, v);
                            Ok(())
                        },
                        None => Err(CE::from_kind(CEK::ConfigOverrideTypeNotMatching)),
                    },
                    None => Err(CE::from_kind(CEK::ConfigOverrideKeyNotAvailable)),
                })
            );

        for elem in iter {
            let _ = try!(elem.chain_err(|| CEK::ConfigOverrideError));
        }

        Ok(())
    }
}

/// Tries to convert the String `s` into the same type as `value`.
///
/// Returns None if string cannot be converted.
///
/// Arrays and Tables are not supported and will yield `None`.
fn into_value(value: &Value, s: String) -> Option<Value> {
    use std::str::FromStr;

    match *value {
        Value::String(_)  => Some(Value::String(s)),
        Value::Integer(_) => FromStr::from_str(&s[..]).ok().map(Value::Integer),
        Value::Float(_)   => FromStr::from_str(&s[..]).ok().map(Value::Float),
        Value::Boolean(_) => {
            if s == "true" { Some(Value::Boolean(true)) }
            else if s == "false" { Some(Value::Boolean(false)) }
            else { None }
        }
        Value::Datetime(_) => Value::try_from(s).ok(),
        _ => None,
    }
}

impl Deref for Configuration {
    type Target = Value;

    fn deref(&self) -> &Value {
        &self.config
    }

}

fn get_verbosity(v: &Value) -> bool {
    match *v {
        Value::Table(ref t) => t.get("verbose")
                .map_or(false, |v| is_match!(v, &Value::Boolean(true))),
        _ => false,
    }
}

fn get_editor(v: &Value) -> Option<String> {
    match *v {
        Value::Table(ref t) => t.get("editor")
                .and_then(|v| match *v { Value::String(ref s) => Some(s.clone()), _ => None, }),
        _ => None,
    }
}

fn get_editor_opts(v: &Value) -> String {
    match *v {
        Value::Table(ref t) => t.get("editor-opts")
                .and_then(|v| match *v { Value::String(ref s) => Some(s.clone()), _ => None, })
                .unwrap_or_default(),
        _ => String::new(),
    }
}

/// Helper to fetch the config file
///
/// Tests several variants for the config file path and uses the first one which works.
fn fetch_config(searchpath: &PathBuf) -> Result<Value> {
    use std::env;
    use std::fs::File;
    use std::io::Read;
    use std::io::Write;
    use std::io::stderr;

    use xdg_basedir;
    use itertools::Itertools;

    use libimagutil::variants::generate_variants as gen_vars;
    use libimagerror::trace::trace_error;

    let variants = vec!["config", "config.toml", "imagrc", "imagrc.toml"];
    let modifier = |base: &PathBuf, v: &'static str| {
        let mut base = base.clone();
        base.push(String::from(v));
        base
    };

    vec![
        vec![searchpath.clone()],
        gen_vars(searchpath.clone(), variants.clone(), &modifier),

        env::var("HOME").map(|home| gen_vars(PathBuf::from(home), variants.clone(), &modifier))
                        .unwrap_or(vec![]),

        xdg_basedir::get_data_home().map(|data_dir| gen_vars(data_dir, variants.clone(), &modifier))
                                    .unwrap_or(vec![]),
    ].iter()
        .flatten()
        .filter(|path| path.exists() && path.is_file())
        .filter_map(|path| {
            let content = {
                let f = File::open(path);
                if f.is_err() {
                    let _ = write!(stderr(), "Error opening file: {:?}", f);
                    return None
                }
                let mut f = f.unwrap();

                let mut s = String::new();
                f.read_to_string(&mut s).ok();
                s
            };

            match ::toml::de::from_str::<::toml::Value>(&content[..]) {
                Ok(res) => Some(res),
                Err(e) => {
                    let line_col = e
                        .line_col()
                        .map(|(line, col)| {
                            format!("Line {}, Column {}", line, col)
                        })
                        .unwrap_or_else(|| String::from("Line unknown, Column unknown"));

                    let _ = write!(stderr(), "Config file parser error at {}", line_col);
                    trace_error(&e);
                    None
                }
            }
        })
        .nth(0)
        .ok_or(CE::from_kind(ConfigErrorKind::NoConfigFileFound))
}

pub trait InternalConfiguration {
    fn enable_logging(&self) -> bool {
        true
    }

    fn use_inmemory_fs(&self) -> bool {
        false
    }
}

impl<'a> InternalConfiguration for App<'a, 'a> {}
