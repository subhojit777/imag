//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015, 2016 Matthias Beyer <mail@beyermatthias.de> and contributors
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; version
// 2.1 of the License.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
//

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
use libimagerror::into::IntoError;

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
            let id = try!(ModuleEntryPath::new(name.clone())
                          .into_storeid()
                          .map_err_into(CEK::StoreWriteError));
            let mut lockentry = try!(store.create(id).map_err_into(CEK::StoreWriteError));

            {
                let mut entry  = lockentry.deref_mut();
                let mut header = entry.get_header_mut();
                let setres = header.set("counter", Value::Table(BTreeMap::new()));
                if setres.is_err() {
                    return Err(CEK::StoreWriteError.into_error());
                }

                let setres = header.set("counter.name", Value::String(name));
                if setres.is_err() {
                    return Err(CEK::StoreWriteError.into_error())
                }

                let setres = header.set("counter.value", Value::Integer(init));
                if setres.is_err() {
                    return Err(CEK::StoreWriteError.into_error())
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
                return Err(CEK::StoreWriteError.into_error())
            }
        };
        Ok(self)
    }

    pub fn inc(&mut self) -> Result<()> {
        let mut header = self.fle.deref_mut().get_header_mut();
        match header.read("counter.value") {
            Ok(Some(Value::Integer(i))) => {
                header.set("counter.value", Value::Integer(i + 1))
                    .map_err_into(CEK::StoreWriteError)
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
                    .map_err_into(CEK::StoreWriteError)
                    .map(|_| ())
            },
            Err(e) => Err(CE::new(CEK::StoreReadError, Some(Box::new(e)))),
            _ => Err(CE::new(CEK::StoreReadError, None)),
        }
    }

    pub fn reset(&mut self) -> Result<()> {
        self.set(0)
    }

    pub fn set(&mut self, v: i64) -> Result<()> {
        let mut header = self.fle.deref_mut().get_header_mut();
        header.set("counter.value", Value::Integer(v))
            .map_err_into(CEK::StoreWriteError)
            .map(|_| ())
    }

    pub fn name(&self) -> Result<CounterName> {
        self.read_header_at("counter.name", |v| match v {
            Some(Value::String(s)) => Ok(s),
            _ => Err(CEK::HeaderTypeError.into_error()),
        })
    }

    pub fn value(&self) -> Result<i64> {
        self.read_header_at("counter.value", |v| match v {
            Some(Value::Integer(i)) => Ok(i),
            _ => Err(CEK::HeaderTypeError.into_error()),
        })
    }

    pub fn unit(&self) -> Option<&CounterUnit> {
        self.unit.as_ref()
    }

    pub fn read_unit(&self) -> Result<Option<CounterUnit>> {
        self.read_header_at("counter.unit", |s| match s {
            Some(Value::String(s)) => Ok(Some(CounterUnit::new(s))),
            Some(_) => Err(CEK::HeaderTypeError.into_error()),
            None => Ok(None),
        })
    }

    fn read_header_at<T, F>(&self, name: &str, f: F) -> Result<T>
        where F: FnOnce(Option<Value>) -> Result<T>
    {
        self.fle.get_header().read(name).map_err_into(CEK::StoreWriteError).and_then(f)
    }

    pub fn load(name: CounterName, store: &Store) -> Result<Counter> {
        debug!("Loading counter: '{}'", name);
        let id = try!(ModuleEntryPath::new(name)
                      .into_storeid()
                      .map_err_into(CEK::StoreWriteError));
        Counter::from_storeid(store, id)
    }

    pub fn delete(name: CounterName, store: &Store) -> Result<()> {
        debug!("Deleting counter: '{}'", name);
        let id = try!(ModuleEntryPath::new(name)
                      .into_storeid()
                      .map_err_into(CEK::StoreWriteError));
        store.delete(id).map_err_into(CEK::StoreWriteError)
    }

    pub fn all_counters(store: &Store) -> Result<CounterIterator> {
        store.retrieve_for_module("counter")
            .map(|iter| CounterIterator::new(store, iter))
            .map_err_into(CEK::StoreReadError)
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

