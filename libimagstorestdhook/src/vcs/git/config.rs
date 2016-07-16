use toml::Value;

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

