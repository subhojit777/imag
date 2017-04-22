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

    /// Apply a `FnMut(Self::Item) -> Result<R, E>` to each item. If each
    /// application returns an `Ok(_)`, return `Ok(())`, indicating success.
    /// Otherwise return the first error.
    ///
    /// The return type of this function only indicates success with the
    /// `Ok(())` idiom. To retrieve the values of your application, include an
    /// accumulator in `func`. This is the intended reason for the permissive
    /// `FnMut` type.
    fn fold_result<R, E, F>(self, mut func: F) -> Result<(), E>
        where F: FnMut(Self::Item) -> Result<R, E>;
}

impl<X, I: Iterator<Item = X>> FoldResult for I {
    type Item = X;

    fn fold_result<R, E, F>(self, mut func: F) -> Result<(), E>
        where F: FnMut(Self::Item) -> Result<R, E>
    {
        for item in self {
            try!(func(item));
        }
        Ok(())
    }
}

#[test]
fn test_fold_result_success() {
    let v = vec![1, 2, 3];
    let mut accum = vec![];
    let result: Result<(), &str> = v.iter().fold_result(|item| {
        accum.push(*item * 2);
        Ok(*item)
    });
    assert_eq!(result, Ok(()));
    assert_eq!(accum, vec![2, 4, 6]);
}

#[test]
fn test_fold_result_failure() {
    let v: Vec<usize> = vec![1, 2, 3];
    let mut accum: Vec<usize> = vec![];
    let result: Result<(), &str> = v.iter().fold_result(|item| if *item == 2 {
        Err("failure")
    } else {
        accum.push(*item * 2);
        Ok(*item)
    });
    assert_eq!(result, Err("failure"));
    assert_eq!(accum, vec![2]);
}
