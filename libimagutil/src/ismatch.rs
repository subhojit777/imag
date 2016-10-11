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

#[macro_export]
macro_rules! is_match {
    ($expression: expr, $($pattern:tt)+) => {
        is_match! {tt
            match $expression {
                $($pattern)+ => true,
                _            => false
            }
        }
    };
    (tt $value:expr) => ($value);
}

#[test]
fn test_matching() {
    let foo = Some("-12");
    assert!(is_match!(foo, Some(bar) if
        is_match!(bar.as_bytes()[0], b'+' | b'-') &&
        is_match!(bar.as_bytes()[1], b'0'...b'9')
    ));
    assert!(!is_match!(foo, None));
}

