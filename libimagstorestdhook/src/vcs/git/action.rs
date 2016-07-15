use std::fmt::{Display, Formatter, Error};

#[derive(Clone, Debug)]
pub enum StoreAction {
    Create,
    Retrieve,
    Update,
    Delete,
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

