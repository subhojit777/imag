use std::default::Default;
use std::fmt::{Debug, Formatter, Error};
use std::path::PathBuf;

pub use config::types::Config;
pub use config::reader::from_file;

pub struct Configuration {
    verbosity: bool,
    editor: Option<String>,
    editor_opts: String,
}

impl Configuration {

    pub fn new(rtp: &PathBuf) -> Option<Configuration> {
        fetch_config(&rtp).and_then(|cfg| {
            let verbosity   = cfg.lookup_boolean("verbosity").unwrap_or(false);
            let editor      = cfg.lookup_str("editor").map(String::from);
            let editor_opts = String::from(cfg.lookup_str("editor-opts").unwrap_or(""));

            debug!("Building configuration");
            debug!("  - verbosity  : {:?}", verbosity);
            debug!("  - editor     : {:?}", editor);
            debug!("  - editor-opts: {}", editor_opts);

            Some(Configuration {
                verbosity: verbosity,
                editor: editor,
                editor_opts: editor_opts,
            })
        })
    }

}

fn fetch_config(rtp: &PathBuf) -> Option<Config> {
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
        .map(|path| from_file(&path).ok())
        .filter(|loaded| loaded.is_some())
        .nth(0)
        .map(|inner| inner.unwrap())
}

impl Debug for Configuration {

    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
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

