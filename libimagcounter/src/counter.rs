use std::ops::DerefMut;

use toml::Value;

use std::collections::BTreeMap;
use std::fmt;
use std::fmt::Display;

use libimagstore::store::Store;
use libimagstore::storeid::StoreIdIterator;
use libimagstore::store::FileLockEntry;
use libimagstore::storeid::StoreId;
use libimagstore::storeid::IntoStoreId;

use module_path::ModuleEntryPath;
use result::Result;
use error::CounterError as CE;
use error::CounterErrorKind as CEK;
use error::error::MapErrInto;

pub type CounterName = String;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct CounterUnit(String);

impl Display for CounterUnit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.0)
    }
}

impl CounterUnit {
    pub fn new<S: Into<String>>(unit: S) -> CounterUnit {
        CounterUnit(unit.into())
    }
}

pub struct Counter<'a> {
    fle: FileLockEntry<'a>,
    unit: Option<CounterUnit>,
}

impl<'a> Counter<'a> {

    pub fn new(store: &Store, name: CounterName, init: i64) -> Result<Counter> {
        use std::ops::DerefMut;

        debug!("Creating new counter: '{}' with value: {}", name, init);
        let fle = {
            let lockentry = store.create(ModuleEntryPath::new(name.clone()).into_storeid());
            if lockentry.is_err() {
                return Err(CE::new(CEK::StoreWriteError, Some(Box::new(lockentry.err().unwrap()))));
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

        Ok(Counter { fle: fle, unit: None })
    }

    pub fn with_unit(mut self, unit: Option<CounterUnit>) -> Result<Counter<'a>> {
        self.unit = unit;

        if let Some(u) = self.unit.clone() {
            let mut header = self.fle.deref_mut().get_header_mut();
            let setres = header.set("counter.unit", Value::String(u.0));
            if setres.is_err() {
                self.unit = None;
                return Err(CE::new(CEK::StoreWriteError, Some(Box::new(setres.unwrap_err()))));
            }
        };
        Ok(self)
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

    pub fn unit(&self) -> Option<&CounterUnit> {
        self.unit.as_ref()
    }

    pub fn read_unit(&self) -> Result<Option<CounterUnit>> {
        self.fle.get_header().read("counter.unit")
            .map_err_into(CEK::StoreReadError)
            .and_then(|s| match s {
                Some(Value::String(s)) => Ok(Some(CounterUnit::new(s))),
                Some(_) => Err(CE::new(CEK::HeaderTypeError, None)),
                None => Ok(None),
            })
    }

    pub fn load(name: CounterName, store: &Store) -> Result<Counter> {
        debug!("Loading counter: '{}'", name);
        let id = ModuleEntryPath::new(name).into_storeid();
        Counter::from_storeid(store, id)
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
    fn from_storeid(&Store, StoreId) -> Result<Counter>;
}

impl<'a> FromStoreId for Counter<'a> {

    fn from_storeid(store: &Store, id: StoreId) -> Result<Counter> {
        debug!("Loading counter from storeid: '{:?}'", id);
        match store.retrieve(id) {
            Err(e) => Err(CE::new(CEK::StoreReadError, Some(Box::new(e)))),
            Ok(c)  => {
                let mut counter = Counter { fle: c, unit: None };
                counter.read_unit()
                    .map_err_into(CEK::StoreReadError)
                    .and_then(|u| {
                        counter.unit = u;
                        Ok(counter)   
                    })
            }
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

