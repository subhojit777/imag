#[derive(Debug, Clone)]
pub enum HookPosition {
    PreRead,
    PostRead,
    PreCreate,
    PostCreate,
    PreRetrieve,
    PostRetrieve,
    PreUpdate,
    PostUpdate,
    PreDelete,
    PostDelete,
}
