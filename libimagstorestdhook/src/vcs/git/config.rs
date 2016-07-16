use toml::Value;

use libimagerror::into::IntoError;

use vcs::git::error::GitHookErrorKind as GHEK;
use vcs::git::error::MapErrInto;
use vcs::git::result::Result;

use vcs::git::action::StoreAction;

pub fn commit_interactive(config: &Value) -> bool {
    unimplemented!()
}

pub fn commit_message(config: &Value, action: StoreAction) -> Option<String> {
    if commit_interactive(config) {
        unimplemented!()
    } else {
        unimplemented!()
    }
}

pub fn abort_on_repo_init_err(cfg: Option<&Value>) -> bool {
    cfg.map(|cfg| {
        match cfg.lookup("abort_on_repo_init_failure") {
            Some(&Value::Boolean(b)) => b,
            Some(_) => {
                warn!("Configuration error, 'abort_on_repo_init_failure' must be a Boolean (true|false).");
                warn!("Assuming 'true' now.");
                true
            },
            None => {
                debug!("No key `abort_on_repo_init_failure' - Assuming 'true'");
                true
            },
        }
    })
    .unwrap_or(false)
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

