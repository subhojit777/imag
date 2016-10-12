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

/// Folds its contents to a result.
pub trait FoldResult: Sized {
    type Item;

    /// Processes all contained items returning the last successful result or the first error.
    /// If there are no items, returns `Ok(R::default())`.
    fn fold_defresult<R, E, F>(self, func: F) -> Result<R, E>
        where R: Default,
              F: FnMut(Self::Item)
        -> Result<R, E>
    {
        self.fold_result(R::default(), func)
    }

    /// Processes all contained items returning the last successful result or the first error.
    /// If there are no items, returns `Ok(default)`.
    fn fold_result<R, E, F>(self, default: R, mut func: F) -> Result<R, E>
        where F: FnMut(Self::Item) -> Result<R, E>;
}

impl<X, I: Iterator<Item = X>> FoldResult for I {
    type Item = X;

    fn fold_result<R, E, F>(self, default: R, mut func: F) -> Result<R, E>
        where F: FnMut(Self::Item) -> Result<R, E>
    {
        self.fold(Ok(default), |acc, item| acc.and_then(|_| func(item)))
    }
}

