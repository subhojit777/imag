pub enum StoreAction {
    Create,
    Retrieve,
    Update,
    Delete,

    // "Read" doesn't matter, as we do not use git on read actions, only when altering content.
}
