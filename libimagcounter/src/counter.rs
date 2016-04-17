use std::ops::DerefMut;

use toml::Value;

use std::collections::BTreeMap;

use libimagstore::store::Store;
use libimagstore::storeid::StoreIdIterator;
use libimagstore::store::FileLockEntry;
use libimagstore::storeid::StoreId;
use libimagstore::storeid::IntoStoreId;

use module_path::ModuleEntryPath;
use result::Result;
use error::CounterError as CE;
use error::CounterErrorKind as CEK;

pub type CounterName = String;

pub struct Counter<'a> {
    fle: FileLockEntry<'a>,
}

impl<'a> Counter<'a> {

    pub fn new(store: &Store, name: CounterName, init: i64) -> Result<Counter> {
        use std::ops::DerefMut;

        debug!("Creating new counter: '{}' with value: {}", name, init);
        let fle = {
            let lockentry = store.create(ModuleEntryPath::new(name.clone()).into_storeid());
            if lockentry.is_err() {
                return Err(CE::new(CEK::StoreWriteError, Some(Box::new(lockentry.unwrap_err()))));
            }
            let mut lockentry = lockentry.unwrap();

            {
                let mut entry  = lockentry.deref_mut();
                let mut header = entry.get_header_mut();
                let setres = header.set("counter", Value::Table(BTreeMap::new()));
                if setres.is_err() {
                    return Err(CE::new(CEK::StoreWriteError, Some(Box::new(setres.unwrap_err()))));
                }

                let setres = header.set("counter.name", Value::String(name));
                if setres.is_err() {
                    return Err(CE::new(CEK::StoreWriteError, Some(Box::new(setres.unwrap_err()))));
                }

                let setres = header.set("counter.value", Value::Integer(init));
                if setres.is_err() {
                    return Err(CE::new(CEK::StoreWriteError, Some(Box::new(setres.unwrap_err()))));
                }
            }

            lockentry
        };

        Ok(Counter { fle: fle })
    }

    pub fn inc(&mut self) -> Result<()> {
        let mut header = self.fle.deref_mut().get_header_mut();
        match header.read("counter.value") {
            Ok(Some(Value::Integer(i))) => {
                header.set("counter.value", Value::Integer(i + 1))
                    .map_err(|e| CE::new(CEK::StoreWriteError, Some(Box::new(e))))
                    .map(|_| ())
            },
            Err(e) => Err(CE::new(CEK::StoreReadError, Some(Box::new(e)))),
            _ => Err(CE::new(CEK::StoreReadError, None)),
        }
    }

    pub fn dec(&mut self) -> Result<()> {
        let mut header = self.fle.deref_mut().get_header_mut();
        match header.read("counter.value") {
            Ok(Some(Value::Integer(i))) => {
                header.set("counter.value", Value::Integer(i - 1))
                    .map_err(|e| CE::new(CEK::StoreWriteError, Some(Box::new(e))))
                    .map(|_| ())
            },
            Err(e) => Err(CE::new(CEK::StoreReadError, Some(Box::new(e)))),
            _ => Err(CE::new(CEK::StoreReadError, None)),
        }
    }

    pub fn reset(&mut self) -> Result<()> {
        let mut header = self.fle.deref_mut().get_header_mut();
        header.set("counter.value", Value::Integer(0))
            .map_err(|e| CE::new(CEK::StoreWriteError, Some(Box::new(e))))
            .map(|_| ())
    }

    pub fn set(&mut self, v: i64) -> Result<()> {
        let mut header = self.fle.deref_mut().get_header_mut();
        header.set("counter.value", Value::Integer(v))
            .map_err(|e| CE::new(CEK::StoreWriteError, Some(Box::new(e))))
            .map(|_| ())
    }

    pub fn name(&self) -> Result<CounterName> {
        self.fle.get_header().read("counter.name")
            .map_err(|e| CE::new(CEK::StoreWriteError, Some(Box::new(e))))
            .and_then(|v| {
                match v {
                    Some(Value::String(s)) => Ok(s),
                    _ => Err(CE::new(CEK::HeaderTypeError, None)),
                }
            })
    }

    pub fn value(&self) -> Result<i64> {
        self.fle.get_header().read("counter.value")
            .map_err(|e| CE::new(CEK::StoreWriteError, Some(Box::new(e))))
            .and_then(|v| {
                match v {
                    Some(Value::Integer(i)) => Ok(i),
                    _ => Err(CE::new(CEK::HeaderTypeError, None)),
                }
            })
    }

    pub fn load(name: CounterName, store: &Store) -> Result<Counter> {
        debug!("Loading counter: '{}'", name);
        match store.retrieve(ModuleEntryPath::new(name).into_storeid()) {
            Err(e) => Err(CE::new(CEK::StoreReadError, Some(Box::new(e)))),
            Ok(c)  => Ok(Counter { fle: c }),
        }
    }

    pub fn delete(name: CounterName, store: &Store) -> Result<()> {
        debug!("Deleting counter: '{}'", name);
        store.delete(ModuleEntryPath::new(name).into_storeid())
            .map_err(|e| CE::new(CEK::StoreWriteError, Some(Box::new(e))))
    }

    pub fn all_counters(store: &Store) -> Result<CounterIterator> {
        store.retrieve_for_module("counter")
            .map(|iter| CounterIterator::new(store, iter))
            .map_err(|e| CE::new(CEK::StoreReadError, Some(Box::new(e))))
    }

}

trait FromStoreId {
    fn from_storeid<'a>(&'a Store, StoreId) -> Result<Counter<'a>>;
}

impl<'a> FromStoreId for Counter<'a> {

    fn from_storeid<'b>(store: &'b Store, id: StoreId) -> Result<Counter<'b>> {
        debug!("Loading counter from storeid: '{:?}'", id);
        match store.retrieve(id) {
            Err(e) => Err(CE::new(CEK::StoreReadError, Some(Box::new(e)))),
            Ok(c)  => Ok(Counter { fle: c }),
        }
    }

}

pub struct CounterIterator<'a> {
    store: &'a Store,
    iditer: StoreIdIterator,
}

impl<'a> CounterIterator<'a> {

    pub fn new(store: &'a Store, iditer: StoreIdIterator) -> CounterIterator<'a> {
        CounterIterator {
            store: store,
            iditer: iditer,
        }
    }

}

impl<'a> Iterator for CounterIterator<'a> {
    type Item = Result<Counter<'a>>;

    fn next(&mut self) -> Option<Result<Counter<'a>>> {
        self.iditer
            .next()
            .map(|id| Counter::from_storeid(self.store, id))
    }

}

