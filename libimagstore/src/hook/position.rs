#[derive(Debug, Clone)]
pub enum HookPosition {
    StoreUnload,

    PreCreate,
    PostCreate,
    PreRetrieve,
    PostRetrieve,
    PreUpdate,
    PostUpdate,
    PreDelete,
    PostDelete,
}
