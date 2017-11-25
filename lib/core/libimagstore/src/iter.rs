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

macro_rules! mk_iterator_mod {
    {
        modname   = $modname:ident,
        itername  = $itername:ident,
        iteryield = $yield:ty,
        extname   = $extname:ident,
        extfnname = $extfnname:ident,
        fun       = $fun:expr
    } => {
        pub mod $modname {
            use storeid::StoreId;
            #[allow(unused_imports)]
            use store::FileLockEntry;
            use store::Store;
            use error::Result;

            pub struct $itername<'a>(Box<Iterator<Item = StoreId>>, &'a Store);

            impl<'a> Iterator for $itername<'a> {
                type Item = Result<$yield>;

                fn next(&mut self) -> Option<Self::Item> {
                    self.0.next().map(|id| $fun(id, self.1))
                }
            }

            pub trait $extname<'a> {
                fn $extfnname(self, store: &'a Store) -> $itername<'a>;
            }

            impl<'a, I> $extname<'a> for I
                where I: Iterator<Item = StoreId> + 'static
            {
                fn $extfnname(self, store: &'a Store) -> $itername<'a> {
                    $itername(Box::new(self), store)
                }
            }

        }
    }
}

mk_iterator_mod! {
    modname   = create,
    itername  = StoreCreateIterator,
    iteryield = FileLockEntry<'a>,
    extname   = StoreIdCreateIteratorExtension,
    extfnname = into_create_iter,
    fun       = |id: StoreId, store: &'a Store| store.create(id)
}

mk_iterator_mod! {
    modname   = delete,
    itername  = StoreDeleteIterator,
    iteryield = (),
    extname   = StoreIdDeleteIteratorExtension,
    extfnname = into_delete_iter,
    fun       = |id: StoreId, store: &'a Store| store.delete(id)
}

mk_iterator_mod! {
    modname   = get,
    itername  = StoreGetIterator,
    iteryield = Option<FileLockEntry<'a>>,
    extname   = StoreIdGetIteratorExtension,
    extfnname = into_get_iter,
    fun       = |id: StoreId, store: &'a Store| store.get(id)
}

mk_iterator_mod! {
    modname   = retrieve,
    itername  = StoreRetrieveIterator,
    iteryield = FileLockEntry<'a>,
    extname   = StoreIdRetrieveIteratorExtension,
    extfnname = into_retrieve_iter,
    fun       = |id: StoreId, store: &'a Store| store.retrieve(id)
}

