//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015-2018 Matthias Beyer <mail@beyermatthias.de> and contributors
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

use filters::filter::Filter;

use libimagstore::storeid::StoreId;

pub struct IsInCollectionsFilter<'a, A>(Option<A>, ::std::marker::PhantomData<&'a str>)
    where A: AsRef<[&'a str]>;

impl<'a, A> IsInCollectionsFilter<'a, A>
    where A: AsRef<[&'a str]>
{
    pub fn new(collections: Option<A>) -> Self {
        IsInCollectionsFilter(collections, ::std::marker::PhantomData)
    }
}

impl<'a, A> Filter<StoreId> for IsInCollectionsFilter<'a, A>
    where A: AsRef<[&'a str]> + 'a
{
    fn filter(&self, sid: &StoreId) -> bool {
        match self.0 {
            Some(ref colls) => sid.is_in_collection(colls),
            None => true,
        }
    }
}

/// Language definition for the header-filter language
///
/// # Notes
///
/// Here are some notes how the language should look like:
///
/// ```ignore
/// query = filter (operator filter)*
///
/// filter = unary? ((function "(" selector ")" ) | selector ) compare_op compare_val
///
/// unary = "not"
///
/// compare_op =
///     "is"     |
///     "in"     |
///     "==/eq"  |
///     "!=/neq" |
///     ">="     |
///     "<="     |
///     "<"      |
///     ">"      |
///     "any"    |
///     "all"
///
/// compare_val = val | listofval
///
/// val         = string | int | float | bool
/// listofval   = "[" (val ",")* "]"
///
/// operator =
///     "or"      |
///     "or_not"  |
///     "and"     |
///     "and_not" |
///     "xor"
///
/// function =
///     "length" |
///     "keys"   |
///     "values"
/// ```
///
mod header_filter_lang {
}

