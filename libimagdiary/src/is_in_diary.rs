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
        self.local().starts_with(format!("diary/{}", name))
    }

}

