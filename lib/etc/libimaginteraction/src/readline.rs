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

use error::InteractionError as IE;
use error::InteractionErrorKind as IEK;
use error::ResultExt;

use toml::Value;

use rustyline::{Config, Editor};

pub struct Readline {
    editor: Editor,
    history_file: PathBuf,
    prompt: String,
}

impl Readline {

    pub fn new(rt: &Runtime) -> Result<Readline> {
        let c = rt.config().ok_or(IEK::NoConfigError)?;

        let histfile     = c.lookup("ui.cli.readline_history_file").ok_or(IEK::ConfigError)?;
        let histsize     = c.lookup("ui.cli.readline_history_size").ok_or(IEK::ConfigError)?;
        let histigndups  = c.lookup("ui.cli.readline_history_ignore_dups").ok_or(IEK::ConfigError)?;
        let histignspace = c.lookup("ui.cli.readline_history_ignore_space").ok_or(IEK::ConfigError)?;
        let prompt       = c.lookup("ui.cli.readline_prompt").ok_or(IEK::ConfigError)?;

        let histfile = histfile
            .as_str()
            .map(PathBuf::from)
            .ok_or(IE::from_kind(IEK::ConfigTypeError))
            .chain_err(|| IEK::ConfigError)
            .chain_err(|| IEK::ReadlineError)?;

        let histsize = histsize
            .as_int()
            .ok_or(IE::from_kind(IEK::ConfigTypeError))
            .chain_err(|| IEK::ConfigError)
            .chain_err(|| IEK::ReadlineError)?;

        let histigndups = histigndups
            .as_bool()
            .ok_or(IE::from_kind(IEK::ConfigTypeError))
            .chain_err(|| IEK::ConfigError)
            .chain_err(|| IEK::ReadlineError)?;

        let histignspace = histignspace
            .as_bool()
            .ok_or(IE::from_kind(IEK::ConfigTypeError))
            .chain_err(|| IEK::ConfigError)
            .chain_err(|| IEK::ReadlineError)?;

        let prompt = prompt
            .as_str()
            .ok_or(IE::from_kind(IEK::ConfigTypeError))
            .chain_err(|| IEK::ConfigError)
            .chain_err(|| IEK::ReadlineError)?;

        let config = Config::builder().
            .max_history_size(histsize)
            .history_ignore_dups(histigndups)
            .history_ignore_space(histignspace)
            .build();

        let mut editor = Editor::new(config);

        if !histfile.exists() {
            let _ = File::create(histfile.clone())
                         .chain_err(|| IEK::ReadlineHistoryFileCreationError)?;
        }

        let _ = editor.load_history(&histfile).chain_err(|| ReadlineError)?;

        Ok(Readline {
            editor: editor,
            history_file: histfile,
            prompt: prompt,
        })
    }

    pub fn read_line(&mut self) -> Result<Option<String>> {
        use rustyline::ReadlineError;
        use libimagutil::warn_result::*;

        match self.editor.readline(&self.prompt) {
            Ok(line) => {
                self.editor.add_history_line(&line);
                self.editor
                    .save_history(&self.history_file)
                    .map_warn_err_str(|e| format!("Could not save history file {} -> {:?}",
                                                  self.history_file.display(), e));
                return line;
            },
            Err(ReadlineError::Interrupted) => {
                info!("CTRL-C");
                Ok(None)
            },
            Err(ReadlineError::Eof) => {
                info!("CTRL-D");
                Ok(None)
            },
            Err(err) => Err(err).map_err_into(ReadlineError),

        }
    }

}

