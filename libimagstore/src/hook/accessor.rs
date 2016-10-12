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

use std::fmt::Debug;

use hook::result::HookResult;
use store::FileLockEntry;
use storeid::StoreId;

pub trait StoreIdAccessor : Debug + Send {
    fn access(&self, &StoreId) -> HookResult<()>;
}

pub trait MutableHookDataAccessor : Debug + Send {
    fn access_mut(&self, &mut FileLockEntry) -> HookResult<()>;
}

pub trait NonMutableHookDataAccessor : Debug + Send {
    fn access(&self, &FileLockEntry) -> HookResult<()>;
}

#[derive(Debug)]
pub enum HookDataAccessor<'a> {
    StoreIdAccess(&'a StoreIdAccessor),
    MutableAccess(&'a MutableHookDataAccessor),
    NonMutableAccess(&'a NonMutableHookDataAccessor),
}

pub trait HookDataAccessorProvider {
    fn accessor(&self) -> HookDataAccessor;
}


