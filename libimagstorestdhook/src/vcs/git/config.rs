use toml::Value;

use libimagerror::into::IntoError;

use vcs::git::error::GitHookErrorKind as GHEK;
use vcs::git::error::MapErrInto;
use vcs::git::result::Result;

use vcs::git::action::StoreAction;

pub fn commit_interactive(config: &Value) -> bool {
    match config.lookup("commit.interactive") {
        Some(&Value::Boolean(b)) => b,
        Some(_) => {
            warn!("Configuration error, 'store.hooks.stdhook_git_update.commit.interactive' must be a Boolean.");
            warn!("Defaulting to commit.interactive = false");
            false
        }
        None => {
            warn!("Unavailable configuration for");
            warn!("\t'store.hooks.stdhook_git_update.commit.interactive'");
            warn!("Defaulting to false");
            false
        }
    }
}

fn commit_with_editor(config: &Value) -> bool {
    match config.lookup("commit.interactive_editor") {
        Some(&Value::Boolean(b)) => b,
        Some(_) => {
            warn!("Configuration error, 'store.hooks.stdhook_git_update.commit.interactive_editor' must be a Boolean.");
            warn!("Defaulting to commit.interactive_editor = false");
            false
        }
        None => {
            warn!("Unavailable configuration for");
            warn!("\t'store.hooks.stdhook_git_update.commit.interactive_editor'");
            warn!("Defaulting to false");
            false
        }
    }
}

fn commit_default_msg<'a>(config: &'a Value) -> &'a str {
    match config.lookup("commit.message") {
        Some(&Value::String(ref b)) => b,
        Some(_) => {
            warn!("Configuration error, 'store.hooks.stdhook_git_update.commit.message' must be a String.");
            warn!("Defaulting to commit.message = 'Update'");
            "Update"
        }
        None => {
            warn!("Unavailable configuration for");
            warn!("\t'store.hooks.stdhook_git_update.commit.message'");
            warn!("Defaulting to commit.message = 'Update'");
            "Update"
        }
    }
}

fn commit_template() -> &'static str {
    "Commit template"
}

pub fn commit_message(repo: &Repository, config: &Value, action: StoreAction) -> Result<String> {
    use libimaginteraction::ask::ask_string;
    use libimagutil::edit::edit_in_tmpfile_with_command;
    use std::process::Command;

    if commit_interactive(config) {
        if commit_with_editor(config) {
            unimplemented!()
        } else {
            unimplemented!()
        }
    } else {
        Ok(String::from(commit_default_msg(config)))
    }
}

pub fn abort_on_repo_init_err(cfg: Option<&Value>) -> bool {
    get_bool_cfg(cfg, "abort_on_repo_init_failure", true, true)
}

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

pub fn do_checkout_ensure_branch(cfg: Option<&Value>) -> bool {
    get_bool_cfg(cfg, "try_checkout_ensure_branch", true, true)
}

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
                debug!("No key '{}' - Assuming '{}'", name, on_unavail);
                on_unavail
            },
        }
    })
    .unwrap_or(on_unavail)
}

