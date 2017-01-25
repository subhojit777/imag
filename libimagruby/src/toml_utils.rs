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

// Toml -> Ruby translation primitives

use ruru::{Object, AnyObject, RString, Fixnum, Float, Boolean, Hash, Array};
use toml::Value;

pub trait AsRuby : Sized {
    fn as_ruby(&self) -> AnyObject;
}

pub trait IntoRuby : AsRuby {
    fn into_ruby(self) -> AnyObject {
        self.as_ruby()
    }
}
impl<T: AsRuby> IntoRuby for T { }

impl AsRuby for Value {

    fn as_ruby(&self) -> AnyObject {
        match *self {
            Value::String(ref s)   => RString::new(&s).to_any_object(),
            Value::Integer(i)      => Fixnum::new(i).to_any_object(),
            Value::Float(f)        => Float::new(f).to_any_object(),
            Value::Boolean(b)      => Boolean::new(b).to_any_object(),
            Value::Datetime(ref s) => RString::new(&s).to_any_object(),
            Value::Array(ref a)    => {
                let mut arr = Array::new();
                for obj in a.into_iter().map(AsRuby::as_ruby) {
                    arr.push(obj);
                }
                arr.to_any_object()
            },
            Value::Table(ref t) => {
                let mut h = Hash::new();
                for (k, v) in t.into_iter() {
                    let key = RString::new(k).to_any_object();
                    let v = v.as_ruby();
                    h.store(key, v);
                }
                h.to_any_object()
            },
        }
    }

}

