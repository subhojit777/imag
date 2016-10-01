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

use toml::Value;

use libimagerror::into::IntoError;
use libimagstore::storeid::StoreId;

use vcs::git::error::GitHookErrorKind as GHEK;
use vcs::git::error::MapErrInto;
use vcs::git::result::Result;

use vcs::git::action::StoreAction;

use git2::Repository;

/// Check the configuration whether we should commit interactively
pub fn commit_interactive(config: &Value, action: &StoreAction) -> bool {
    match config.lookup("commit.interactive") {
        Some(&Value::Boolean(b)) => b,
        Some(_) => {
            warn!("Configuration error, 'store.hooks.stdhook_git_{}.commit.interactive' must be a Boolean.",
                  action);
            warn!("Defaulting to commit.interactive = false");
            false
        }
        None => {
            warn!("Unavailable configuration for");
            warn!("\t'store.hooks.stdhook_git_{}.commit.interactive'", action);
            warn!("Defaulting to false");
            false
        }
    }
}

/// Check the configuration whether we should commit with the editor
fn commit_with_editor(config: &Value, action: &StoreAction) -> bool {
    match config.lookup("commit.interactive_editor") {
        Some(&Value::Boolean(b)) => b,
        Some(_) => {
            warn!("Configuration error, 'store.hooks.stdhook_git_{}.commit.interactive_editor' must be a Boolean.",
                  action);
            warn!("Defaulting to commit.interactive_editor = false");
            false
        }
        None => {
            warn!("Unavailable configuration for");
            warn!("\t'store.hooks.stdhook_git_{}.commit.interactive_editor'", action);
            warn!("Defaulting to false");
            false
        }
    }
}

/// Get the commit default message
fn commit_default_msg<'a>(config: &'a Value, action: &'a StoreAction) -> &'a str {
    match config.lookup("commit.message") {
        Some(&Value::String(ref b)) => b,
        Some(_) => {
            warn!("Configuration error, 'store.hooks.stdhook_git_{}.commit.message' must be a String.",
                  action);
            warn!("Defaulting to commit.message = '{}'", action.as_commit_message());
            action.as_commit_message()
        }
        None => {
            warn!("Unavailable configuration for");
            warn!("\t'store.hooks.stdhook_git_{}.commit.message'", action);
            warn!("Defaulting to commit.message = '{}'", action.as_commit_message());
            action.as_commit_message()
        }
    }
}

/// Get the commit template
///
/// TODO: Implement good template string
fn commit_template(action: &StoreAction, id: &StoreId) -> String {
    format!(r#"
# Please commit your changes and remove these lines.
#
# You're about to commit changes via the {action} Hook
#
#   Altered file: {id}
#
    "#,
    action = action,
    id = id.local().display())
}

/// Generate a commit message
///
/// Uses the functions `commit_interactive()` and `commit_with_editor()`
/// or reads one from the commandline or uses the `commit_default_msg()` string to create a commit
/// message.
pub fn commit_message(repo: &Repository, config: &Value, action: StoreAction, id: &StoreId) -> Result<String> {
    use libimaginteraction::ask::ask_string;
    use libimagutil::edit::edit_in_tmpfile_with_command;
    use std::process::Command;

    if commit_interactive(config, &action) {
        if commit_with_editor(config, &action) {
            repo.config()
                .map_err_into(GHEK::GitConfigFetchError)
                .and_then(|c| c.get_string("core.editor").map_err_into(GHEK::GitConfigEditorFetchError))
                .map_err_into(GHEK::ConfigError)
                .map(Command::new)
                .and_then(|cmd| {
                    let mut s = commit_template(&action, id);
                    edit_in_tmpfile_with_command(cmd, &mut s).map(|_| s)
                        .map_err_into(GHEK::EditorError)
                })
        } else {
            Ok(ask_string("Commit Message", None, false, false, None, "> "))
        }
    } else {
        Ok(String::from(commit_default_msg(config, &action)))
    }
}

/// Check whether the hook should abort if the repository cannot be initialized
pub fn abort_on_repo_init_err(cfg: &Value) -> bool {
    get_bool_cfg(Some(cfg), "abort_on_repo_init_failure", true, true)
}

/// Get the branch which must be checked out before running the hook (if any).
///
/// If there is no configuration for this, this is `Ok(None)`, otherwise we try to find the
/// configuration `String`.
pub fn ensure_branch(cfg: Option<&Value>) -> Result<Option<String>> {
    match cfg {
        Some(cfg) => {
            match cfg.lookup("ensure_branch") {
                Some(&Value::String(ref s)) => Ok(Some(s.clone())),
                Some(_) => {
                    warn!("Configuration error, 'ensure_branch' must be a String.");
                    Err(GHEK::ConfigTypeError.into_error())
                        .map_err_into(GHEK::ConfigTypeError)
                },
                None => {
                    debug!("No key `ensure_branch'");
                    Ok(None)
                },
            }
        },
        None => Ok(None),
    }
}

/// Check whether we should check out a branch before committing.
pub fn do_checkout_ensure_branch(cfg: Option<&Value>) -> bool {
    get_bool_cfg(cfg, "try_checkout_ensure_branch", true, true)
}

/// Helper to get a boolean value from the configuration.
fn get_bool_cfg(cfg: Option<&Value>, name: &str, on_fail: bool, on_unavail: bool) -> bool {
    cfg.map(|cfg| {
        match cfg.lookup(name) {
            Some(&Value::Boolean(b)) => b,
            Some(_) => {
                warn!("Configuration error, '{}' must be a Boolean (true|false).", name);
                warn!("Assuming '{}' now.", on_fail);
                on_fail
            },
            None => {
                warn!("No key '{}' - Assuming '{}'", name, on_unavail);
                on_unavail
            },
        }
    })
    .unwrap_or_else(|| {
        warn!("No configuration to fetch {} from, assuming {}", name, on_unavail);
        on_unavail
    })
}

/// Check whether the hook is enabled or not. If the config is not there, the hook is _enabled_ by
/// default.
pub fn is_enabled(cfg: &Value) -> bool {
    get_bool_cfg(Some(cfg), "enabled", true, true)
}

