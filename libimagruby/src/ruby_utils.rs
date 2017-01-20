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

// Ruby -> Toml translation primitives

use std::collections::BTreeMap;

use ruru::{Object, AnyObject, Class, RString, Fixnum, Float, Symbol, Hash, Array, VM};
use ruru::types::ValueType;
use toml::Value;


pub trait AsToml : Sized {
    fn as_toml(&self) -> Value;
}

pub trait IntoToml : AsToml {
    fn into_toml(self) -> Value {
        self.as_toml()
    }
}
impl<T: AsToml> IntoToml for T { }

impl AsToml for AnyObject {

    fn as_toml(&self) -> Value {
        match self.value().ty() {
             ValueType::None => {
                 Value::Boolean(false)
             },
             ValueType::Object => {
                 let rte = Class::from_existing("TypeError");
                 VM::raise(rte, "Cannot translate type '' to fit into TOML");
                 Value::Boolean(false)
             },
             ValueType::Class => {
                 let rte = Class::from_existing("TypeError");
                 VM::raise(rte, "Cannot translate type '' to fit into TOML");
                 Value::Boolean(false)
             },
             ValueType::Module => {
                 let rte = Class::from_existing("TypeError");
                 VM::raise(rte, "Cannot translate type '' to fit into TOML");
                 Value::Boolean(false)
             },
             ValueType::Float =>
                        Value::Float(self.try_convert_to::<Float>().unwrap().to_f64()),
             ValueType::RString =>
                        Value::String(self.try_convert_to::<RString>().unwrap().to_string()),
             ValueType::Regexp => {
                 let rte = Class::from_existing("TypeError");
                 VM::raise(rte, "Cannot translate type '' to fit into TOML");
                 Value::Boolean(false)
             },
             ValueType::Array => {
                let vals = self
                    .try_convert_to::<Array>()
                    .unwrap()
                    .into_iter()
                    .map(|v| v.as_toml())
                    .collect::<Vec<Value>>();

                Value::Array(vals)
             },
             ValueType::Hash => {
                let mut btm = BTreeMap::new();
                self.try_convert_to::<Hash>()
                    .unwrap()
                    .each(|key, value| {
                        let key = match key.as_toml() {
                            Value::String(s) => s,
                            _ => {
                                let rte = Class::from_existing("TypeError");
                                VM::raise(rte, "Can only have String or Symbol as Key for TOML maps");
                                String::new()
                            }
                        };
                        let value = value.as_toml();
                        btm.insert(key, value);
                    });
                Value::Table(btm)
             },
             ValueType::Struct => {
                 let rte = Class::from_existing("TypeError");
                 VM::raise(rte, "Cannot translate type '' to fit into TOML");
                 Value::Boolean(false)
             },
             ValueType::Bignum => {
                 let rte = Class::from_existing("TypeError");
                 VM::raise(rte, "Cannot translate type '' to fit into TOML");
                 Value::Boolean(false)
             },
             ValueType::File => {
                 let rte = Class::from_existing("TypeError");
                 VM::raise(rte, "Cannot translate type '' to fit into TOML");
                 Value::Boolean(false)
             },
             ValueType::Data => {
                 let rte = Class::from_existing("TypeError");
                 VM::raise(rte, "Cannot translate type '' to fit into TOML");
                 Value::Boolean(false)
             },
             ValueType::Match => {
                 let rte = Class::from_existing("TypeError");
                 VM::raise(rte, "Cannot translate type '' to fit into TOML");
                 Value::Boolean(false)
             },
             ValueType::Complex => {
                 let rte = Class::from_existing("TypeError");
                 VM::raise(rte, "Cannot translate type '' to fit into TOML");
                 Value::Boolean(false)
             },
             ValueType::Rational => {
                 let rte = Class::from_existing("TypeError");
                 VM::raise(rte, "Cannot translate type '' to fit into TOML");
                 Value::Boolean(false)
             },
             ValueType::Nil => Value::Boolean(false),
             ValueType::True => Value::Boolean(true),
             ValueType::False => Value::Boolean(false),
             ValueType::Symbol => Value::String(self.try_convert_to::<Symbol>().unwrap().to_string()),
             ValueType::Fixnum => Value::Integer(self.try_convert_to::<Fixnum>().unwrap().to_i64()),
             ValueType::Undef => {
                 let rte = Class::from_existing("TypeError");
                 VM::raise(rte, "Cannot translate type '' to fit into TOML");
                 Value::Boolean(false)
             },
             ValueType::Node => {
                 let rte = Class::from_existing("TypeError");
                 VM::raise(rte, "Cannot translate type '' to fit into TOML");
                 Value::Boolean(false)
             },
             ValueType::IClass => {
                 let rte = Class::from_existing("TypeError");
                 VM::raise(rte, "Cannot translate type '' to fit into TOML");
                 Value::Boolean(false)
             },
             ValueType::Zombie => {
                 let rte = Class::from_existing("TypeError");
                 VM::raise(rte, "Cannot translate type '' to fit into TOML");
                 Value::Boolean(false)
             },
             ValueType::Mask => {
                 let rte = Class::from_existing("TypeError");
                 VM::raise(rte, "Cannot translate type '' to fit into TOML");
                 Value::Boolean(false)
             },
        }
    }

}

