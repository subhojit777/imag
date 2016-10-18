//
// imag - the personal information management suite for the commandline
// Copyright (C) 2016 the imag contributors
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

use std::error::Error;

/// An iterator that maps `f` over the `Error` elements of `iter`, similar to `std::iter::Map`.
///
/// This `struct` is created by the `on_err()` method on `TraceIterator`. See its
/// documentation for more information.
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct OnErr<I, F>{
    iter: I,
    f: F
}

impl<I, F, T, E> Iterator for OnErr<I, F> where
    I: Iterator<Item = Result<T, E>>,
    F: FnMut(&E)
{
    type Item = Result<T, E>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(Err(e)) => {
                (self.f)(&e);
                Some(Err(e))
            },
            other => other
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<I, F> ExactSizeIterator for OnErr<I, F> where
    I: ExactSizeIterator,
    OnErr<I, F>: Iterator
{
}

impl<I, F, T, E> DoubleEndedIterator for OnErr<I, F> where
    I: DoubleEndedIterator<Item = Result<T, E>>,
    F: FnMut(&E)
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.iter.next_back() {
            Some(Err(e)) => {
                (self.f)(&e);
                Some(Err(e))
            },
            other => other
        }
    }
}

/// An iterator that unwraps the `Ok` items of `iter`, while passing the `Err` items to its
/// closure `f`.
///
/// This `struct` is created by the `unwrap_with()` method on `TraceIterator`. See its
/// documentation for more information.
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct UnwrapWith<I, F>{
    iter: I,
    f: F
}

impl<I, F, T, E> Iterator for UnwrapWith<I, F> where
    I: Iterator<Item = Result<T, E>>,
    F: FnMut(E)
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                Some(Err(e)) => {
                    (self.f)(e);
                },
                Some(Ok(item)) => return Some(item),
                None => return None,
            }
        }
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_, upper) = self.iter.size_hint();
        (0, upper)
    }
}

impl<I, F, T, E> DoubleEndedIterator for UnwrapWith<I, F> where
    I: DoubleEndedIterator<Item = Result<T, E>>,
    F: FnMut(E)
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next_back() {
                Some(Err(e)) => {
                    (self.f)(e);
                },
                Some(Ok(item)) => return Some(item),
                None => return None,
            }
        }
    }
}

/// This trait provides methods that make it easier to work with iterators that yield a `Result`.
pub trait TraceIterator<T, E> : Iterator<Item = Result<T, E>> + Sized {
    /// Creates an iterator that yields the item in each `Ok` item, while filtering out the `Err`
    /// items. Each filtered `Err` will be trace-logged with [`::trace::trace_error`].
    ///
    /// As with all iterators, the processing is lazy. If you do not use the result of this method,
    /// nothing will be passed to `::trace::trace_error`, no matter how many `Err` items might
    /// be present.
    #[inline]
    fn trace_unwrap(self) -> UnwrapWith<Self, fn(E)> where E: Error {
        #[inline]
        fn trace_error<E: Error>(err: E) {
            ::trace::trace_error(&err);
        }

        self.unwrap_with(trace_error)
    }

    /// Takes a closure and creates an iterator that will call that closure for each `Err` element.
    /// The resulting iterator will yield the exact same items as the original iterator. A close
    /// analogue from the standard library would be `Iterator::inspect`.
    ///
    /// As with all iterators, the processing is lazy. The result of this method must be evaluated
    /// for the closure to be called.
    #[inline]
    fn on_err<F>(self, f: F) -> OnErr<Self, F>  where F: FnMut(&E) {
        OnErr { iter: self, f: f }
    }

    /// Takes a closure and creates an iterator that will yield the items inside all `Ok` items
    /// yielded by the original iterator. All `Err` items will be filtered out, and the contents
    /// of each `Err` will be passed to the closure.
    ///
    /// As with all iterators, the processing is lazy. The result of this method must be evaluated
    /// for the closure to be called.
    #[inline]
    fn unwrap_with<F>(self, f: F) -> UnwrapWith<Self, F>
        where F: FnMut(E)
    {
        UnwrapWith { iter: self, f: f }
    }
}

impl<I, T, E> TraceIterator<T, E> for I where
    I: Iterator<Item = Result<T, E>>
{}

#[cfg(test)]
mod test {
    use super::TraceIterator;
    
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    struct TestError(i32);
    
    #[test]
    fn test_unwrap_with() {
        let original = vec![Ok(1), Err(TestError(2)), Ok(3), Err(TestError(4))];
        let mut errs = vec![];

        let oks = original
            .into_iter()
            .unwrap_with(|e|errs.push(e))
            .collect::<Vec<_>>();

        assert_eq!(&oks, &[1, 3]);
        assert_eq!(&errs, &[TestError(2), TestError(4)]);
    }

    #[test]
    fn test_unwrap_with_backward() {
        let original = vec![Ok(1), Err(TestError(2)), Ok(3), Err(TestError(4))];
        let mut errs = vec![];

        let oks = original
            .into_iter()
            .rev()
            .unwrap_with(|e|errs.push(e))
            .collect::<Vec<_>>();

        assert_eq!(&oks, &[3, 1]);
        assert_eq!(&errs, &[TestError(4), TestError(2)]);
    }

    #[test]
    fn test_on_err() {
        let original = vec![Ok(1), Err(TestError(2)), Ok(3), Err(TestError(4))];
        let mut errs = vec![];

        let result = original
            .into_iter()
            .on_err(|e|errs.push(e.clone()))
            .collect::<Vec<_>>();

        assert_eq!(&result, &[Ok(1), Err(TestError(2)), Ok(3), Err(TestError(4))]);
        assert_eq!(&errs, &[TestError(2), TestError(4)]);
     }

    #[test]
    fn test_on_err_backward() {
        let original = vec![Ok(1), Err(TestError(2)), Ok(3), Err(TestError(4))];
        let mut errs = vec![];

        let result = original
            .into_iter()
            .rev()
            .on_err(|e|errs.push(e.clone()))
            .collect::<Vec<_>>();

        assert_eq!(&result, &[Err(TestError(4)), Ok(3), Err(TestError(2)), Ok(1)]);
        assert_eq!(&errs, &[TestError(4), TestError(2)]);
     }
}
