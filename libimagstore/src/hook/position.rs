#[derive(Debug, Clone)]
pub enum HookPosition {
    PreCreate,
    PostCreate,
    PreRetrieve,
    PostRetrieve,
    PreUpdate,
    PostUpdate,
    PreDelete,
    PostDelete,
}
