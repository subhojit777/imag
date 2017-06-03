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
use std::process::Command;
use std::env;
use std::io::stderr;
use std::io::Write;

pub use clap::App;

use clap::{Arg, ArgMatches};
use log;
use log::LogLevelFilter;

use configuration::Configuration;
use error::RuntimeError;
use error::RuntimeErrorKind;
use error::MapErrInto;
use logger::ImagLogger;

use libimagstore::store::Store;

/// The Runtime object
///
/// This object contains the complete runtime environment of the imag application running.
#[derive(Debug)]
pub struct Runtime<'a> {
    rtp: PathBuf,
    configuration: Option<Configuration>,
    cli_matches: ArgMatches<'a>,
    store: Store,
}

impl<'a> Runtime<'a> {

    /// Gets the CLI spec for the program and retreives the config file path (or uses the default on
    /// in $HOME/.imag/config, $XDG_CONFIG_DIR/imag/config or from env("$IMAG_CONFIG")
    /// and builds the Runtime object with it.
    ///
    /// The cli_spec object should be initially build with the ::get_default_cli_builder() function.
    pub fn new(mut cli_spec: App<'a, 'a>) -> Result<Runtime<'a>, RuntimeError> {
        use std::env;
        use std::io::stdout;

        use clap::Shell;

        use libimagerror::trace::trace_error;
        use libimagerror::into::IntoError;

        use configuration::error::ConfigErrorKind;

        let matches = cli_spec.clone().get_matches();

        let is_debugging = matches.is_present("debugging");
        let is_verbose   = matches.is_present("verbosity");
        let colored      = !matches.is_present("no-color-output");

        Runtime::init_logger(is_debugging, is_verbose, colored);

        match matches.value_of(Runtime::arg_generate_compl()) {
            Some(shell) => {
                debug!("Generating shell completion script, writing to stdout");
                let shell   = shell.parse::<Shell>().unwrap(); // clap has our back here.
                let appname = String::from(cli_spec.get_name());
                cli_spec.gen_completions_to(appname, shell, &mut stdout());
            },
            _ => debug!("Not generating shell completion script"),
        }

        let rtp : PathBuf = matches.value_of("runtimepath")
            .map_or_else(|| {
                env::var("HOME")
                    .map(PathBuf::from)
                    .map(|mut p| { p.push(".imag"); p})
                    .unwrap_or_else(|_| {
                        panic!("You seem to be $HOME-less. Please get a $HOME before using this software. We are sorry for you and hope you have some accommodation anyways.");
                    })
            }, PathBuf::from);
        let storepath = matches.value_of("storepath")
                                .map_or_else(|| {
                                    let mut spath = rtp.clone();
                                    spath.push("store");
                                    spath
                                }, PathBuf::from);

        let configpath = matches.value_of("config")
                                .map_or_else(|| rtp.clone(), PathBuf::from);

        debug!("RTP path    = {:?}", rtp);
        debug!("Store path  = {:?}", storepath);
        debug!("Config path = {:?}", configpath);

        let cfg = match Configuration::new(&configpath) {
            Err(e) => if e.err_type() != ConfigErrorKind::NoConfigFileFound {
                return Err(RuntimeErrorKind::Instantiate.into_error_with_cause(Box::new(e)));
            } else {
                warn!("No config file found.");
                warn!("Continuing without configuration file");
                None
            },

            Ok(mut cfg) => {
                if let Err(e) = cfg.override_config(get_override_specs(&matches)) {
                    error!("Could not apply config overrides");
                    trace_error(&e);

                    // TODO: continue question (interactive)
                }

                Some(cfg)
            }
        };

        let store_config = match cfg {
            Some(ref c) => c.store_config().cloned(),
            None        => None,
        };

        if is_debugging {
            write!(stderr(), "Config: {:?}\n", cfg).ok();
            write!(stderr(), "Store-config: {:?}\n", store_config).ok();
        }

        Store::new(storepath.clone(), store_config).map(|store| {
            Runtime {
                cli_matches: matches,
                configuration: cfg,
                rtp: rtp,
                store: store,
            }
        })
        .map_err_into(RuntimeErrorKind::Instantiate)
    }

    ///
    /// Get a commandline-interface builder object from `clap`
    ///
    /// This commandline interface builder object already contains some predefined interface flags:
    ///   * -v | --verbose for verbosity
    ///   * --debug for debugging
    ///   * -c <file> | --config <file> for alternative configuration file
    ///   * -r <path> | --rtp <path> for alternative runtimepath
    ///   * --store <path> for alternative store path
    /// Each has the appropriate help text included.
    ///
    /// The `appname` shall be "imag-<command>".
    ///
    pub fn get_default_cli_builder(appname: &'a str,
                                   version: &'a str,
                                   about: &'a str)
        -> App<'a, 'a>
    {
        App::new(appname)
            .version(version)
            .author("Matthias Beyer <mail@beyermatthias.de>")
            .about(about)
            .arg(Arg::with_name(Runtime::arg_verbosity_name())
                .short("v")
                .long("verbose")
                .help("Enables verbosity")
                .required(false)
                .takes_value(false))

            .arg(Arg::with_name(Runtime::arg_debugging_name())
                .long("debug")
                .help("Enables debugging output")
                .required(false)
                .takes_value(false))

            .arg(Arg::with_name(Runtime::arg_no_color_output_name())
                .long("no-color")
                .help("Disable color output")
                .required(false)
                .takes_value(false))

            .arg(Arg::with_name(Runtime::arg_config_name())
                .long("config")
                .help("Path to alternative config file")
                .required(false)
                .takes_value(true))

            .arg(Arg::with_name(Runtime::arg_config_override_name())
                 .long("override-config")
                 .help("Override a configuration settings. Use 'key=value' pairs, where the key is a path in the TOML configuration. The value must be present in the configuration and be convertible to the type of the configuration setting. If the argument does not contain a '=', it gets ignored. Setting Arrays and Tables is not yet supported.")
                 .required(false)
                 .takes_value(true))

            .arg(Arg::with_name(Runtime::arg_runtimepath_name())
                .long("rtp")
                .help("Alternative runtimepath")
                .required(false)
                .takes_value(true))

            .arg(Arg::with_name(Runtime::arg_storepath_name())
                .long("store")
                .help("Alternative storepath. Must be specified as full path, can be outside of the RTP")
                .required(false)
                .takes_value(true))

            .arg(Arg::with_name(Runtime::arg_editor_name())
                .long("editor")
                .help("Set editor")
                .required(false)
                .takes_value(true))

            .arg(Arg::with_name(Runtime::arg_generate_compl())
                .long("generate-commandline-completion")
                .help("Generate the commandline completion for bash or zsh or fish")
                .required(false)
                .takes_value(true)
                .value_name("SHELL")
                .possible_values(&["bash", "fish", "zsh"]))

    }

    /// Get the argument names of the Runtime which are available
    pub fn arg_names() -> Vec<&'static str> {
        vec![
            Runtime::arg_verbosity_name(),
            Runtime::arg_debugging_name(),
            Runtime::arg_no_color_output_name(),
            Runtime::arg_config_name(),
            Runtime::arg_config_override_name(),
            Runtime::arg_runtimepath_name(),
            Runtime::arg_storepath_name(),
            Runtime::arg_editor_name(),
        ]
    }

    /// Get the verbosity argument name for the Runtime
    pub fn arg_verbosity_name() -> &'static str {
        "verbosity"
    }

    /// Get the debugging argument name for the Runtime
    pub fn arg_debugging_name() -> &'static str {
        "debugging"
    }

    /// Get the argument name for no color output of the Runtime
    pub fn arg_no_color_output_name() -> &'static str {
        "no-color-output"
    }

    /// Get the config argument name for the Runtime
    pub fn arg_config_name() -> &'static str {
        "config"
    }

    /// Get the config-override argument name for the Runtime
    pub fn arg_config_override_name() -> &'static str {
        "config-override"
    }

    /// Get the runtime argument name for the Runtime
    pub fn arg_runtimepath_name() -> &'static str {
        "runtimepath"
    }

    /// Get the storepath argument name for the Runtime
    pub fn arg_storepath_name() -> &'static str {
        "storepath"
    }

    /// Get the editor argument name for the Runtime
    pub fn arg_editor_name() -> &'static str {
        "editor"
    }

    /// Get the argument name for generating the completion
    pub fn arg_generate_compl() -> &'static str {
        "generate-completion"
    }

    /// Initialize the internal logger
    fn init_logger(is_debugging: bool, is_verbose: bool, colored: bool) {
        use std::env::var as env_var;
        use env_logger;

        if env_var("IMAG_LOG_ENV").is_ok() {
            env_logger::init().unwrap();
        } else {
            let lvl = if is_debugging {
                LogLevelFilter::Debug
            } else if is_verbose {
                LogLevelFilter::Info
            } else {
                LogLevelFilter::Warn
            };

            log::set_logger(|max_log_lvl| {
                max_log_lvl.set(lvl);
                debug!("Init logger with {}", lvl);
                Box::new(ImagLogger::new(lvl.to_log_level().unwrap()).with_color(colored))
            })
            .map_err(|e| panic!("Could not setup logger: {:?}", e))
            .ok();
        }
    }

    /// Get the verbosity flag value
    pub fn is_verbose(&self) -> bool {
        self.cli_matches.is_present("verbosity")
    }

    /// Get the debugging flag value
    pub fn is_debugging(&self) -> bool {
        self.cli_matches.is_present("debugging")
    }

    /// Get the runtimepath
    pub fn rtp(&self) -> &PathBuf {
        &self.rtp
    }

    /// Get the commandline interface matches
    pub fn cli(&self) -> &ArgMatches {
        &self.cli_matches
    }

    /// Get the configuration object
    pub fn config(&self) -> Option<&Configuration> {
        self.configuration.as_ref()
    }

    /// Get the store object
    pub fn store(&self) -> &Store {
        &self.store
    }

    /// Change the store backend to stdout
    ///
    /// For the documentation on purpose and cavecats, have a look at the documentation of the
    /// `Store::reset_backend()` function.
    ///
    pub fn store_backend_to_stdio(&mut self) -> Result<(), RuntimeError> {
        use libimagstore::file_abstraction::stdio::*;
        use libimagstore::file_abstraction::stdio::mapper::json::JsonMapper;
        use std::rc::Rc;
        use std::cell::RefCell;

        let mut input = ::std::io::stdin();
        let output    = ::std::io::stdout();
        let output    = Rc::new(RefCell::new(output));
        let mapper    = JsonMapper::new();

        StdIoFileAbstraction::new(&mut input, output, mapper)
            .map_err_into(RuntimeErrorKind::Instantiate)
            .and_then(|backend| {
                self.store
                    .reset_backend(Box::new(backend))
                    .map_err_into(RuntimeErrorKind::Instantiate)
            })
    }

    pub fn store_backend_to_stdout(&mut self) -> Result<(), RuntimeError> {
        use libimagstore::file_abstraction::stdio::mapper::json::JsonMapper;
        use libimagstore::file_abstraction::stdio::out::StdoutFileAbstraction;
        use std::rc::Rc;
        use std::cell::RefCell;

        let output    = ::std::io::stdout();
        let output    = Rc::new(RefCell::new(output));
        let mapper    = JsonMapper::new();

        StdoutFileAbstraction::new(output, mapper)
            .map_err_into(RuntimeErrorKind::Instantiate)
            .and_then(|backend| {
                self.store
                    .reset_backend(Box::new(backend))
                    .map_err_into(RuntimeErrorKind::Instantiate)
            })
    }

    /// Get a editor command object which can be called to open the $EDITOR
    pub fn editor(&self) -> Option<Command> {
        self.cli()
            .value_of("editor")
            .map(String::from)
            .or({
                match self.configuration {
                    Some(ref c) => c.editor().cloned(),
                    _ => None,
                }
            })
            .or(env::var("EDITOR").ok())
            .map(Command::new)
    }
}

fn get_override_specs(matches: &ArgMatches) -> Vec<String> {
    matches
        .values_of("config-override")
        .map(|values| {
             values
             .filter(|s| {
                 let b = s.contains("=");
                 if !b { warn!("override '{}' does not contain '=' - will be ignored!", s); }
                 b
             })
             .map(String::from)
             .collect()
        })
        .unwrap_or(vec![])
}

