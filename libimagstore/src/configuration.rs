use toml::Value;
use hook::position::HookPosition;

pub fn config_is_valid(config: &Value) -> bool {
    unimplemented!()
}

pub fn get_pre_read_aspect_names(value: &Value) -> Vec<String> {
    get_aspect_names_for_aspect_position(HookPosition::PreRead, value)
}

pub fn get_post_read_aspect_names(value: &Value) -> Vec<String> {
    get_aspect_names_for_aspect_position(HookPosition::PostRead, value)
}

pub fn get_pre_create_aspect_names(value: &Value) -> Vec<String> {
    get_aspect_names_for_aspect_position(HookPosition::PreCreate, value)
}

pub fn get_post_create_aspect_names(value: &Value) -> Vec<String> {
    get_aspect_names_for_aspect_position(HookPosition::PostCreate, value)
}

pub fn get_pre_retrieve_aspect_names(value: &Value) -> Vec<String> {
    get_aspect_names_for_aspect_position(HookPosition::PreRetrieve, value)
}

pub fn get_post_retrieve_aspect_names(value: &Value) -> Vec<String> {
    get_aspect_names_for_aspect_position(HookPosition::PostRetrieve, value)
}

pub fn get_pre_update_aspect_names(value: &Value) -> Vec<String> {
    get_aspect_names_for_aspect_position(HookPosition::PreUpdate, value)
}

pub fn get_post_update_aspect_names(value: &Value) -> Vec<String> {
    get_aspect_names_for_aspect_position(HookPosition::PostUpdate, value)
}

pub fn get_pre_delete_aspect_names(value: &Value) -> Vec<String> {
    get_aspect_names_for_aspect_position(HookPosition::PreDelete, value)
}

pub fn get_post_delete_aspect_names(value: &Value) -> Vec<String> {
    get_aspect_names_for_aspect_position(HookPosition::PostDelete, value)
}

fn get_aspect_names_for_aspect_position(position: HookPosition, value: &Value) -> Vec<String> {
    unimplemented!()
}
