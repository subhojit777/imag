use libimagstore::store::Entry;
use libimagstore::storeid::StoreId;

pub trait IsInDiary {

    fn is_in_diary(&self, name: &str) -> bool;

}

impl IsInDiary for Entry {

    fn is_in_diary(&self, name: &str) -> bool {
        self.get_location().clone().is_in_diary(name)
    }

}

impl IsInDiary for StoreId {

    fn is_in_diary(&self, name: &str) -> bool {
        use std::path::PathBuf;
        self.is_in_collection(&PathBuf::from(format!("diary/{}", name)))
    }

}

