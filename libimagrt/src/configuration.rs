use std::path::PathBuf;
use std::result::Result as RResult;

use toml::{Parser, Value};

/**
 * Errors which are related to configuration-file loading
 */
pub mod error {
    use std::error::Error;
    use std::fmt::{Debug, Display, Formatter};
    use std::fmt::Error as FmtError;

    /**
     * The kind of an error
     */
    #[derive(Clone, Debug, PartialEq)]
    pub enum ConfigErrorKind {
        ConfigParsingFailed,
        NoConfigFileFound,
    }

    /**
     * Configuration error type
     */
    #[derive(Debug)]
    pub struct ConfigError {
        kind: ConfigErrorKind,
        cause: Option<Box<Error>>,
    }

    impl ConfigError {

        /**
         * Instantiate a new ConfigError, optionally with cause
         */
        pub fn new(kind: ConfigErrorKind, cause: Option<Box<Error>>) -> ConfigError {
            ConfigError {
                kind: kind,
                cause: cause,
            }
        }

        /**
         * get the Kind of the Error
         */
        pub fn kind(&self) -> ConfigErrorKind {
            self.kind.clone()
        }

        /**
         * Get the string, the ConfigError can be described with
         */
        pub fn as_str(e: &ConfigError) -> &'static str {
            match e.kind() {
                ConfigErrorKind::ConfigParsingFailed => "Config parsing failed",
                ConfigErrorKind::NoConfigFileFound   => "No config file found",
            }
        }

    }

    impl Display for ConfigError {

        fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
            try!(write!(fmt, "{}", ConfigError::as_str(self)));
            Ok(())
        }

    }

    impl Error for ConfigError {

        fn description(&self) -> &str {
            ConfigError::as_str(self)
        }

        fn cause(&self) -> Option<&Error> {
            self.cause.as_ref().map(|e| &**e)
        }

    }

}

use self::error::{ConfigError, ConfigErrorKind};


/**
 * Result type of this module. Either T or ConfigError
 */
pub type Result<T> = RResult<T, ConfigError>;

/**
 * Configuration object
 *
 * Holds all config variables which are globally available plus the configuration object from the
 * config parser, which can be accessed.
 */
#[derive(Debug)]
pub struct Configuration {

    /**
     * The plain configuration object for direct access if necessary
     */
    config: Value,

    /**
     * The verbosity the program should run with
     */
    verbosity: bool,

    /**
     * The editor which should be used
     */
    editor: Option<String>,

    /**
     * The options the editor should get when opening some file
     */
    editor_opts: String,
}

impl Configuration {

    /**
     * Get a new configuration object.
     *
     * The passed runtimepath is used for searching the configuration file, whereas several file
     * names are tested. If that does not work, the home directory and the XDG basedir are tested
     * with all variants.
     *
     * If that doesn't work either, an error is returned.
     */
    pub fn new(rtp: &PathBuf) -> Result<Configuration> {
        fetch_config(&rtp).map(|cfg| {
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

    pub fn editor(&self) -> Option<&String> {
        self.editor.as_ref()
    }

    pub fn config(&self) -> &Value {
        &self.config
    }

    pub fn store_config(&self) -> Option<&Value> {
        match &self.config {
            &Value::Table(ref tabl) => tabl.get("store"),
            _ => None,
        }
    }

}

fn get_verbosity(v: &Value) -> bool {
    match v {
        &Value::Table(ref t) => t.get("verbose")
                .map(|v| match v { &Value::Boolean(b) => b, _ => false, })
                .unwrap_or(false),
        _ => false,
    }
}

fn get_editor(v: &Value) -> Option<String> {
    match v {
        &Value::Table(ref t) => t.get("editor")
                .and_then(|v| match v { &Value::String(ref s) => Some(s.clone()), _ => None, }),
        _ => None,
    }
}

fn get_editor_opts(v: &Value) -> String {
    match v {
        &Value::Table(ref t) => t.get("editor-opts")
                .and_then(|v| match v { &Value::String(ref s) => Some(s.clone()), _ => None, })
                .unwrap_or(String::new()),
        _ => String::new(),
    }
}

/**
 * Helper to fetch the config file
 *
 * Tests several variants for the config file path and uses the first one which works.
 */
fn fetch_config(rtp: &PathBuf) -> Result<Value> {
    use std::env;
    use std::fs::File;
    use std::io::Read;

    use xdg_basedir;
    use itertools::Itertools;

    use libimagutil::variants::generate_variants as gen_vars;

    let variants = vec!["config", "config.toml", "imagrc", "imagrc.toml"];
    let modifier = |base: &PathBuf, v: &'static str| {
        let mut base = base.clone();
        base.push(format!("{}", v));
        base
    };

    vec![
        gen_vars(rtp.clone(), variants.clone(), &modifier),

        env::var("HOME").map(|home| gen_vars(PathBuf::from(home), variants.clone(), &modifier))
                        .unwrap_or(vec![]),

        xdg_basedir::get_data_home().map(|data_dir| gen_vars(data_dir, variants.clone(), &modifier))
                                    .unwrap_or(vec![]),
    ].iter()
        .flatten()
        .filter(|path| path.exists() && path.is_file())
        .map(|path| {
            let content = {
                let mut s = String::new();
                let mut f = File::open(path);
                if f.is_err() {
                }
                let mut f = f.unwrap();
                f.read_to_string(&mut s);
                s
            };
            Parser::new(&content[..]).parse()
        })
        .filter(|loaded| loaded.is_some())
        .nth(0)
        .map(|inner| Value::Table(inner.unwrap()))
        .ok_or(ConfigError::new(ConfigErrorKind::NoConfigFileFound, None))
}

