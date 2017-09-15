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

pub mod create {
    use storeid::StoreIdIterator;
    use store::FileLockEntry;
    use store::Store;
    use error::Result;

    pub struct StoreCreateIterator<'a>(StoreIdIterator, &'a Store);

    impl<'a> Iterator for StoreCreateIterator<'a> {
        type Item = Result<FileLockEntry<'a>>;

        fn next(&mut self) -> Option<Self::Item> {
            self.0.next().map(|id| self.1.create(id))
        }
    }

    pub trait StoreIdCreateIteratorExtension<'a> {
        fn into_create_iter(self, store: &'a Store) -> StoreCreateIterator<'a>;
    }

    impl<'a> StoreIdCreateIteratorExtension<'a> for StoreIdIterator {
        fn into_create_iter(self, store: &'a Store) -> StoreCreateIterator<'a> {
            StoreCreateIterator(self, store)
        }
    }
}

pub mod delete {
    use storeid::StoreIdIterator;
    use store::Store;
    use error::Result;

    pub struct StoreDeleteIterator<'a>(StoreIdIterator, &'a Store);

    impl<'a> Iterator for StoreDeleteIterator<'a> {
        type Item = Result<()>;

        fn next(&mut self) -> Option<Self::Item> {
            self.0.next().map(|id| self.1.delete(id))
        }
    }

    pub trait StoreIdDeleteIteratorExtension<'a> {
        fn into_delete_iter(self, store: &'a Store) -> StoreDeleteIterator<'a>;
    }

    impl<'a> StoreIdDeleteIteratorExtension<'a> for StoreIdIterator {
        fn into_delete_iter(self, store: &'a Store) -> StoreDeleteIterator<'a> {
            StoreDeleteIterator(self, store)
        }
    }
}

pub mod get {
    use storeid::StoreIdIterator;
    use store::FileLockEntry;
    use store::Store;
    use error::Result;

    pub struct StoreGetIterator<'a>(StoreIdIterator, &'a Store);

    impl<'a> Iterator for StoreGetIterator<'a> {
        type Item = Result<Option<FileLockEntry<'a>>>;

        fn next(&mut self) -> Option<Self::Item> {
            self.0.next().map(|id| self.1.get(id))
        }
    }

    pub trait StoreIdGetIteratorExtension<'a> {
        fn into_get_iter(self, store: &'a Store) -> StoreGetIterator<'a>;
    }

    impl<'a> StoreIdGetIteratorExtension<'a> for StoreIdIterator {
        fn into_get_iter(self, store: &'a Store) -> StoreGetIterator<'a> {
            StoreGetIterator(self, store)
        }
    }
}

pub mod retrieve {
    use storeid::StoreIdIterator;
    use store::FileLockEntry;
    use store::Store;
    use error::Result;

    pub struct StoreRetrieveIterator<'a>(StoreIdIterator, &'a Store);

    impl<'a> Iterator for StoreRetrieveIterator<'a> {
        type Item = Result<FileLockEntry<'a>>;

        fn next(&mut self) -> Option<Self::Item> {
            self.0.next().map(|id| self.1.retrieve(id))
        }
    }

    pub trait StoreIdRetrieveIteratorExtension<'a> {
        fn into_retrieve_iter(self, store: &'a Store) -> StoreRetrieveIterator<'a>;
    }

    impl<'a> StoreIdRetrieveIteratorExtension<'a> for StoreIdIterator {
        fn into_retrieve_iter(self, store: &'a Store) -> StoreRetrieveIterator<'a> {
            StoreRetrieveIterator(self, store)
        }
    }
}

