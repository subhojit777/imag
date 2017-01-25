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

use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::Mutex;

use uuid::Uuid;

use libimagstore::store::Store;

#[derive(Clone, Debug, Ord, Hash, Eq, PartialOrd, PartialEq)]
pub struct StoreHandle(Uuid);

impl StoreHandle {
    pub fn new() -> StoreHandle {
        StoreHandle(Uuid::new_v4())
    }
}

lazy_static! {
    pub static ref RUBY_STORE_CACHE: Arc<Mutex<BTreeMap<StoreHandle, Store>>> = {
        Arc::new(Mutex::new(BTreeMap::new()))
    };
}

