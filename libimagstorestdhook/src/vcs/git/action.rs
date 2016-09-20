use std::fmt::{Display, Formatter, Error};

/// Utility type to specify which kind of store action is running
#[derive(Clone, Debug)]
pub enum StoreAction {
    Create,
    Retrieve,
    Update,
    Delete,
}

impl StoreAction {

    pub fn uppercase(&self) -> &str {
        match *self {
            StoreAction::Create   => "CREATE",
            StoreAction::Retrieve => "RETRIEVE",
            StoreAction::Update   => "UPDATE",
            StoreAction::Delete   => "DELETE",
        }
    }

    pub fn as_commit_message(&self) -> &str {
        match *self {
            StoreAction::Create   => "Create",
            StoreAction::Retrieve => "Retrieve",
            StoreAction::Update   => "Update",
            StoreAction::Delete   => "Delete",
        }
    }
}

impl Display for StoreAction {

    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "StoreAction: {}",
                match *self {
                    StoreAction::Create   => "create",
                    StoreAction::Retrieve => "retrieve",
                    StoreAction::Update   => "update",
                    StoreAction::Delete   => "delete",
                })
    }

}

