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
