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

macro_rules! mk_iterator {
    {
        modname   = $modname:ident,
        itername  = $itername:ident,
        iteryield = $yield:ty,
        extname   = $extname:ident,
        extfnname = $extfnname:ident,
        fun       = $fun:expr
    } => {
        use storeid::StoreId;
        #[allow(unused_imports)]
        use store::FileLockEntry;
        use store::Store;
        use error::Result;

        pub struct $itername<'a>(Box<Iterator<Item = StoreId> + 'a>, &'a Store);

        impl<'a> $itername<'a> {
            pub fn new(inner: Box<Iterator<Item = StoreId> + 'a>, store: &'a Store) -> Self {
                $itername(inner, store)
            }
        }

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
            where I: Iterator<Item = StoreId> + 'a
        {
            fn $extfnname(self, store: &'a Store) -> $itername<'a> {
                $itername(Box::new(self), store)
            }
        }
    }
}

use error::StoreError;

pub enum ExtensionError<E> {
    Forwarded(E),
    StoreError(StoreError)
}

macro_rules! mk_iterator_mod {
    {
        modname        = $modname:ident,
        itername       = $itername:ident,
        iteryield      = $yield:ty,
        extname        = $extname:ident,
        extfnname      = $extfnname:ident,
        fun            = $fun:expr,
        resultitername = $resultitername:ident,
        resultextname  = $resultextname:ident
    } => {
        pub mod $modname {
            mk_iterator! {
                modname   = $modname,
                itername  = $itername,
                iteryield = $yield,
                extname   = $extname,
                extfnname = $extfnname,
                fun       = $fun
            }

            use std::result::Result as RResult;

            pub struct $resultitername<'a, I>(I, &'a Store);

            impl<'a, I, E> Iterator for $resultitername<'a, I>
                where I: Iterator<Item = RResult<StoreId, E>>
            {
                type Item = RResult<$yield, $crate::iter::ExtensionError<E>>;

                fn next(&mut self) -> Option<Self::Item> {
                    match self.0.next() {
                        Some(Ok(sid)) => Some($fun(sid, self.1).map_err($crate::iter::ExtensionError::StoreError)),
                        Some(Err(e))  => Some(Err($crate::iter::ExtensionError::Forwarded(e))),
                        None => None,
                    }
                }
            }

            pub trait $resultextname<'a> : Iterator {
                fn $extfnname(self, store: &'a Store) -> $resultitername<'a, Self>
                    where Self: Sized
                {
                    $resultitername(self, store)
                }
            }

            impl<'a, I> $resultextname<'a> for I
                where I: Iterator
            { /* empty */ }
        }
    };

    {
        modname   = $modname:ident,
        itername  = $itername:ident,
        iteryield = $yield:ty,
        extname   = $extname:ident,
        extfnname = $extfnname:ident,
        fun       = $fun:expr
    } => {
        pub mod $modname {
            mk_iterator! {
                modname   = $modname,
                itername  = $itername,
                iteryield = $yield,
                extname   = $extname,
                extfnname = $extfnname,
                fun       = $fun
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
    fun       = |id: StoreId, store: &'a Store| store.create(id),
    resultitername = StoreCreateResultIterator,
    resultextname  = StoreIdCreateResultIteratorExtension
}

mk_iterator_mod! {
    modname   = delete,
    itername  = StoreDeleteIterator,
    iteryield = (),
    extname   = StoreIdDeleteIteratorExtension,
    extfnname = into_delete_iter,
    fun       = |id: StoreId, store: &'a Store| store.delete(id),
    resultitername = StoreDeleteResultIterator,
    resultextname  = StoreIdDeleteResultIteratorExtension
}

mk_iterator_mod! {
    modname   = get,
    itername  = StoreGetIterator,
    iteryield = Option<FileLockEntry<'a>>,
    extname   = StoreIdGetIteratorExtension,
    extfnname = into_get_iter,
    fun       = |id: StoreId, store: &'a Store| store.get(id),
    resultitername = StoreGetResultIterator,
    resultextname  = StoreIdGetResultIteratorExtension
}

mk_iterator_mod! {
    modname   = retrieve,
    itername  = StoreRetrieveIterator,
    iteryield = FileLockEntry<'a>,
    extname   = StoreIdRetrieveIteratorExtension,
    extfnname = into_retrieve_iter,
    fun       = |id: StoreId, store: &'a Store| store.retrieve(id),
    resultitername = StoreRetrieveResultIterator,
    resultextname  = StoreIdRetrieveResultIteratorExtension
}

#[cfg(test)]
#[allow(dead_code)]
mod compile_test {

    // This module contains code to check whether this actually compiles the way we would like it to
    // compile

    use store::Store;
    use storeid::StoreId;

    fn store() -> Store {
        unimplemented!("Not implemented because in compile-test")
    }

    fn test_compile_get() {
        let store = store();
        let _ = store
            .entries()
            .unwrap()
            .into_get_iter();
    }

    fn test_compile_get_result() {
        fn to_result(e: StoreId) -> Result<StoreId, ()> {
            Ok(e)
        }

        let store = store();
        let _ = store
            .entries()
            .unwrap()
            .into_get_iter();
    }
}

