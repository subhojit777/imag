use std::default::Default;
use std::fmt::{Debug, Display, Formatter, Error};
use std::path::PathBuf;
use std::result::Result as RResult;

pub use config::types::Config;
pub use config::reader::from_file;

pub mod error {
    use std::error::Error;
    use std::fmt::{Debug, Display, Formatter};
    use std::fmt::Error as FmtError;

    #[derive(Clone, Debug, PartialEq)]
    pub enum ConfigErrorKind {
        ConfigNotFound,
        ConfigParsingFailed,
        NoConfigFileFound,
    }

    #[derive(Debug)]
    pub struct ConfigError {
        kind: ConfigErrorKind,
        cause: Option<Box<Error>>,
    }

    impl ConfigError {

        pub fn new(kind: ConfigErrorKind, cause: Option<Box<Error>>) -> ConfigError {
            ConfigError {
                kind: kind,
                cause: cause,
            }
        }

        pub fn kind(&self) -> ConfigErrorKind {
            self.kind.clone()
        }

        pub fn as_str(e: &ConfigError) -> &'static str {
            match e.kind() {
                ConfigErrorKind::ConfigNotFound      => "Config not found",
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

pub type Result<T> = RResult<T, ConfigError>;

pub struct Configuration {
    verbosity: bool,
    editor: Option<String>,
    editor_opts: String,
}

impl Configuration {

    pub fn new(rtp: &PathBuf) -> Result<Configuration> {
        fetch_config(&rtp).map(|cfg| {
            let verbosity   = cfg.lookup_boolean("verbosity").unwrap_or(false);
            let editor      = cfg.lookup_str("editor").map(String::from);
            let editor_opts = String::from(cfg.lookup_str("editor-opts").unwrap_or(""));

            debug!("Building configuration");
            debug!("  - verbosity  : {:?}", verbosity);
            debug!("  - editor     : {:?}", editor);
            debug!("  - editor-opts: {}", editor_opts);

            Configuration {
                verbosity: verbosity,
                editor: editor,
                editor_opts: editor_opts,
            }
        })
    }

}

fn fetch_config(rtp: &PathBuf) -> Result<Config> {
    use std::process::exit;
    use std::env;

    use xdg_basedir;
    use itertools::Itertools;

    use libimagutil::variants::generate_variants as gen_vars;

    let variants = vec!["config", "config.toml", "imagrc", "imagrc.toml"];
    let modifier = |base: &PathBuf, v: &'static str| {
        let mut base = base.clone();
        base.push(format!("/{}", v));
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
        .filter(|path| path.exists())
        .map(|path| {
            from_file(&path)
                    .map_err(|e| {
                        ConfigError::new(ConfigErrorKind::ConfigParsingFailed, Some(Box::new(e)))
                    })
        })
        .filter(|loaded| loaded.is_ok())
        .nth(0)
        .map(|inner| inner.unwrap())
        .ok_or(ConfigError::new(ConfigErrorKind::NoConfigFileFound, None))
}

impl Debug for Configuration {

    fn fmt(&self, f: &mut Formatter) -> RResult<(), Error> {
        try!(write!(f, "Configuration (verbose: {})", self.verbosity));
        Ok(())
    }

}

impl Default for Configuration {

    fn default() -> Configuration {
        Configuration {
            verbosity: false,
            editor: Some(String::from("nano")),
            editor_opts: String::from(""),
        }
    }

}

