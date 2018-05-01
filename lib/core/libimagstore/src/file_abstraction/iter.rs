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

use std::path::PathBuf;
use std::sync::Arc;

use error::Result;
use storeid::StoreId;
use file_abstraction::FileAbstraction;

/// A wrapper for an iterator over `PathBuf`s
pub struct PathIterator(Box<Iterator<Item = Result<PathBuf>>>);

impl PathIterator {

    pub fn new(iter: Box<Iterator<Item = Result<PathBuf>>>) -> PathIterator {
        PathIterator(iter)
    }

    pub fn store_id_constructing(self, storepath: PathBuf, backend: Arc<FileAbstraction>)
        -> StoreIdConstructingIterator
    {
        StoreIdConstructingIterator(self, storepath, backend)
    }

}

impl Iterator for PathIterator {
    type Item = Result<PathBuf>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

}


/// Helper type for constructing StoreIds from a PathIterator.
///
/// Automatically ignores non-files.
pub struct StoreIdConstructingIterator(PathIterator, PathBuf, Arc<FileAbstraction>);

impl Iterator for StoreIdConstructingIterator {
    type Item = Result<StoreId>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(next) = self.0.next() {
            match next {
                Err(e)  => return Some(Err(e)),
                Ok(next) => match self.2.exists(&next) {
                    Err(e)    => return Some(Err(e)),
                    Ok(true)  => return Some(StoreId::from_full_path(&self.1, next)),
                    Ok(false) => { continue },
                }
            }
        }

        None
    }

}

