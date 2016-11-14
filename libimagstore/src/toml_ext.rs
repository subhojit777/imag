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

use toml::Value;

use store::Result;
use error::{StoreError as SE, StoreErrorKind as SEK};
use libimagerror::into::IntoError;

pub trait TomlValueExt {
    fn insert_with_sep(&mut self, spec: &str, sep: char, v: Value) -> Result<bool>;
    fn set_with_sep(&mut self, spec: &str, sep: char, v: Value) -> Result<Option<Value>>;
    fn read_with_sep(&self, spec: &str, splitchr: char) -> Result<Option<Value>>;
    fn delete_with_sep(&mut self, spec: &str, splitchr: char) -> Result<Option<Value>>;

    #[inline]
    fn insert(&mut self, spec: &str, v: Value) -> Result<bool> {
        self.insert_with_sep(spec, '.', v)
    }

    #[inline]
    fn set(&mut self, spec: &str, v: Value) -> Result<Option<Value>> {
        self.set_with_sep(spec, '.', v)
    }

    #[inline]
    fn read(&self, spec: &str) -> Result<Option<Value>> {
        self.read_with_sep(spec, '.')
    }

    #[inline]
    fn delete(&mut self, spec: &str) -> Result<Option<Value>> {
        self.delete_with_sep(spec, '.')
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Key(String),
    Index(usize),
}

impl TomlValueExt for Value {
    /**
     * Insert a header field by a string-spec
     *
     * ```ignore
     *  insert("something.in.a.field", Boolean(true));
     * ```
     *
     * If an array field was accessed which is _out of bounds_ of the array available, the element
     * is appended to the array.
     *
     * Inserts a Boolean in the section "something" -> "in" -> "a" -> "field"
     * A JSON equivalent would be
     *
     *  {
     *      something: {
     *          in: {
     *              a: {
     *                  field: true
     *              }
     *          }
     *      }
     *  }
     *
     * Returns true if header field was set, false if there is already a value
     */
    fn insert_with_sep(&mut self, spec: &str, sep: char, v: Value) -> Result<bool> {
        let tokens = match tokenize(spec, sep) {
            Err(e) => return Err(e),
            Ok(t) => t
        };

        let destination = match tokens.iter().last() {
            None => return Err(SE::new(SEK::HeaderPathSyntaxError, None)),
            Some(d) => d,
        };

        let path_to_dest = tokens[..(tokens.len() - 1)].into(); // N - 1 tokens

        // walk N-1 tokens
        let value = match walk_header(self, path_to_dest) {
            Err(e) => return Err(e),
            Ok(v) => v
        };

        // There is already an value at this place
        if extract(value, destination).is_ok() {
            return Ok(false);
        }

        match *destination {
            Token::Key(ref s) => { // if the destination shall be an map key
                match *value {
                    /*
                     * Put it in there if we have a map
                     */
                    Value::Table(ref mut t) => {
                        t.insert(s.clone(), v);
                    }

                    /*
                     * Fail if there is no map here
                     */
                    _ => return Err(SEK::HeaderPathTypeFailure.into_error()),
                }
            },

            Token::Index(i) => { // if the destination shall be an array
                match *value {

                    /*
                     * Put it in there if we have an array
                     */
                    Value::Array(ref mut a) => {
                        a.push(v); // push to the end of the array

                        // if the index is inside the array, we swap-remove the element at this
                        // index
                        if a.len() < i {
                            a.swap_remove(i);
                        }
                    },

                    /*
                     * Fail if there is no array here
                     */
                    _ => return Err(SEK::HeaderPathTypeFailure.into_error()),
                }
            },
        }

        Ok(true)
    }

    /**
     * Set a header field by a string-spec
     *
     * ```ignore
     *  set("something.in.a.field", Boolean(true));
     * ```
     *
     * Sets a Boolean in the section "something" -> "in" -> "a" -> "field"
     * A JSON equivalent would be
     *
     *  {
     *      something: {
     *          in: {
     *              a: {
     *                  field: true
     *              }
     *          }
     *      }
     *  }
     *
     * If there is already a value at this place, this value will be overridden and the old value
     * will be returned
     */
    fn set_with_sep(&mut self, spec: &str, sep: char, v: Value) -> Result<Option<Value>> {
        let tokens = match tokenize(spec, sep) {
            Err(e) => return Err(e),
            Ok(t) => t,
        };
        debug!("tokens = {:?}", tokens);

        let destination = match tokens.iter().last() {
            None => return Err(SEK::HeaderPathSyntaxError.into_error()),
            Some(d) => d
        };
        debug!("destination = {:?}", destination);

        let path_to_dest = tokens[..(tokens.len() - 1)].into(); // N - 1 tokens
        // walk N-1 tokens
        let value = match walk_header(self, path_to_dest) {
            Err(e) => return Err(e),
            Ok(v) => v
        };
        debug!("walked value = {:?}", value);

        match *destination {
            Token::Key(ref s) => { // if the destination shall be an map key->value
                match *value {
                    /*
                     * Put it in there if we have a map
                     */
                    Value::Table(ref mut t) => {
                        debug!("Matched Key->Table");
                        return Ok(t.insert(s.clone(), v));
                    }

                    /*
                     * Fail if there is no map here
                     */
                    _ => {
                        debug!("Matched Key->NON-Table");
                        return Err(SEK::HeaderPathTypeFailure.into_error());
                    }
                }
            },

            Token::Index(i) => { // if the destination shall be an array
                match *value {

                    /*
                     * Put it in there if we have an array
                     */
                    Value::Array(ref mut a) => {
                        debug!("Matched Index->Array");
                        a.push(v); // push to the end of the array

                        // if the index is inside the array, we swap-remove the element at this
                        // index
                        if a.len() > i {
                            debug!("Swap-Removing in Array {:?}[{:?}] <- {:?}", a, i, a[a.len()-1]);
                            return Ok(Some(a.swap_remove(i)));
                        }

                        debug!("Appended");
                        return Ok(None);
                    },

                    /*
                     * Fail if there is no array here
                     */
                    _ => {
                        debug!("Matched Index->NON-Array");
                        return Err(SEK::HeaderPathTypeFailure.into_error());
                    },
                }
            },
        }

        Ok(None)
    }

    /**
     * Read a header field by a string-spec
     *
     * ```ignore
     *  let value = read("something.in.a.field");
     * ```
     *
     * Reads a Value in the section "something" -> "in" -> "a" -> "field"
     * A JSON equivalent would be
     *
     *  {
     *      something: {
     *          in: {
     *              a: {
     *                  field: true
     *              }
     *          }
     *      }
     *  }
     *
     * If there is no a value at this place, None will be returned. This also holds true for Arrays
     * which are accessed at an index which is not yet there, even if the accessed index is much
     * larger than the array length.
     */
    fn read_with_sep(&self, spec: &str, splitchr: char) -> Result<Option<Value>> {
        let tokens = match tokenize(spec, splitchr) {
            Err(e) => return Err(e),
            Ok(t) => t,
        };

        let mut header_clone = self.clone(); // we clone as READing is simpler this way
        // walk N-1 tokens
        match walk_header(&mut header_clone, tokens) {
            Err(e) => match e.err_type() {
                // We cannot find the header key, as there is no path to it
                SEK::HeaderKeyNotFound => Ok(None),
                _ => Err(e),
            },
            Ok(v) => Ok(Some(v.clone())),
        }
    }

    fn delete_with_sep(&mut self, spec: &str, splitchr: char) -> Result<Option<Value>> {
        let tokens = match tokenize(spec, splitchr) {
            Err(e) => return Err(e),
            Ok(t) => t
        };

        let destination = match tokens.iter().last() {
            None => return Err(SEK::HeaderPathSyntaxError.into_error()),
            Some(d) => d
        };
        debug!("destination = {:?}", destination);

        let path_to_dest = tokens[..(tokens.len() - 1)].into(); // N - 1 tokens
        // walk N-1 tokens
        let mut value = match walk_header(self, path_to_dest) {
            Err(e) => return Err(e),
            Ok(v) => v
        };
        debug!("walked value = {:?}", value);

        match *destination {
            Token::Key(ref s) => { // if the destination shall be an map key->value
                match *value {
                    Value::Table(ref mut t) => {
                        debug!("Matched Key->Table, removing {:?}", s);
                        return Ok(t.remove(s));
                    },
                    _ => {
                        debug!("Matched Key->NON-Table");
                        return Err(SEK::HeaderPathTypeFailure.into_error());
                    }
                }
            },

            Token::Index(i) => { // if the destination shall be an array
                match *value {
                    Value::Array(ref mut a) => {
                        // if the index is inside the array, we swap-remove the element at this
                        // index
                        if a.len() > i {
                            debug!("Removing in Array {:?}[{:?}]", a, i);
                            return Ok(Some(a.remove(i)));
                        } else {
                            return Ok(None);
                        }
                    },
                    _ => {
                        debug!("Matched Index->NON-Array");
                        return Err(SEK::HeaderPathTypeFailure.into_error());
                    },
                }
            },
        }

        Ok(None)
    }

}

fn tokenize(spec: &str, splitchr: char) -> Result<Vec<Token>> {
    use std::str::FromStr;

    spec.split(splitchr)
        .map(|s| {
            usize::from_str(s)
                .map(Token::Index)
                .or_else(|_| Ok(Token::Key(String::from(s))))
        })
        .collect()
}

fn walk_header(v: &mut Value, tokens: Vec<Token>) -> Result<&mut Value> {
    use std::vec::IntoIter;

    fn walk_iter<'a>(v: Result<&'a mut Value>, i: &mut IntoIter<Token>) -> Result<&'a mut Value> {
        let next = i.next();
        v.and_then(move |value| {
            if let Some(token) = next {
                walk_iter(extract(value, &token), i)
            } else {
                Ok(value)
            }
        })
    }

    walk_iter(Ok(v), &mut tokens.into_iter())
}

fn extract_from_table<'a>(v: &'a mut Value, s: &str) -> Result<&'a mut Value> {
    match *v {
        Value::Table(ref mut t) => {
            t.get_mut(&s[..])
                .ok_or(SEK::HeaderKeyNotFound.into_error())
        },
        _ => Err(SEK::HeaderPathTypeFailure.into_error()),
    }
}

fn extract_from_array(v: &mut Value, i: usize) -> Result<&mut Value> {
    match *v {
        Value::Array(ref mut a) => {
            if a.len() < i {
                Err(SEK::HeaderKeyNotFound.into_error())
            } else {
                Ok(&mut a[i])
            }
        },
        _ => Err(SEK::HeaderPathTypeFailure.into_error()),
    }
}

fn extract<'a>(v: &'a mut Value, token: &Token) -> Result<&'a mut Value> {
    match *token {
        Token::Key(ref s)  => extract_from_table(v, s),
        Token::Index(i)    => extract_from_array(v, i),
    }
}

