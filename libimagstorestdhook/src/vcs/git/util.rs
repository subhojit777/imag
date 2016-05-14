//! Utility functionality for integrating git hooks in the store
//!
//! Contains primitives to create a repository within the store path

pub fn mkrepo(store: &Store) -> Result<()> {
    unimplemented!()
}

pub fn hasrepo(store: &Store) -> bool {
    Repository::open(store.path()).is_ok()
}

