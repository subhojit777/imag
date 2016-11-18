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

use std::result::Result as RResult;
use std::collections::BTreeMap;

use toml::{Table, Value};

use store::Result;
use error::StoreError as SE;
use error::StoreErrorKind as SEK;
use error::{ParserErrorKind, ParserError};
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
        let (destination, value) = try!(setup(self, spec, sep));

        // There is already an value at this place
        if value.extract(&destination).is_ok() {
            return Ok(false);
        }

        match destination {
            // if the destination shall be an map key
            Token::Key(ref s) => match *value {
                /*
                 * Put it in there if we have a map
                 */
                Value::Table(ref mut t) => { t.insert(s.clone(), v); },

                /*
                 * Fail if there is no map here
                 */
                _ => return Err(SEK::HeaderPathTypeFailure.into_error()),
            },

            // if the destination shall be an array
            Token::Index(i) => match *value {

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
        let (destination, value) = try!(setup(self, spec, sep));

        match destination {
            // if the destination shall be an map key->value
            Token::Key(ref s) => match *value {
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
            },

            // if the destination shall be an array
            Token::Index(i) => match *value {

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
            },
        }
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
        let tokens = try!(tokenize(spec, splitchr));

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
        let (destination, value) = try!(setup(self, spec, splitchr));

        match destination {
            // if the destination shall be an map key->value
            Token::Key(ref s) => match *value {
                Value::Table(ref mut t) => {
                    debug!("Matched Key->Table, removing {:?}", s);
                    return Ok(t.remove(s));
                },
                _ => {
                    debug!("Matched Key->NON-Table");
                    return Err(SEK::HeaderPathTypeFailure.into_error());
                }
            },

            // if the destination shall be an array
            Token::Index(i) => match *value {

                // if the index is inside the array, we swap-remove the element at this
                // index
                Value::Array(ref mut a) => if a.len() > i {
                    debug!("Removing in Array {:?}[{:?}]", a, i);
                    return Ok(Some(a.remove(i)));
                } else {
                    return Ok(None);
                },
                _ => {
                    debug!("Matched Index->NON-Array");
                    return Err(SEK::HeaderPathTypeFailure.into_error());
                },
            },
        }
    }

}

fn setup<'a>(v: &'a mut Value, spec: &str, sep: char)
    -> Result<(Token, &'a mut Value)>
{
    let tokens       = try!(tokenize(spec, sep));
    debug!("tokens = {:?}", tokens);

    let destination  = try!(tokens.iter().last().cloned().ok_or(SEK::HeaderPathSyntaxError.into_error()));
    debug!("destination = {:?}", destination);

    let path_to_dest : Vec<Token> = tokens[..(tokens.len() - 1)].into(); // N - 1 tokens
    let value        = try!(walk_header(v, path_to_dest)); // walk N-1 tokens

    debug!("walked value = {:?}", value);

    Ok((destination, value))
}

fn tokenize(spec: &str, splitchr: char) -> Result<Vec<Token>> {
    use std::str::FromStr;

    spec.split(splitchr)
        .map(|s| usize::from_str(s).map(Token::Index).or_else(|_| Ok(Token::Key(String::from(s)))))
        .collect()
}

fn walk_header(v: &mut Value, tokens: Vec<Token>) -> Result<&mut Value> {
    use std::vec::IntoIter;

    fn walk_iter<'a>(v: Result<&'a mut Value>, i: &mut IntoIter<Token>) -> Result<&'a mut Value> {
        let next = i.next();
        v.and_then(move |value| if let Some(token) = next {
            walk_iter(value.extract(&token), i)
        } else {
            Ok(value)
        })
    }

    walk_iter(Ok(v), &mut tokens.into_iter())
}

trait Extract {
    fn extract<'a>(&'a mut self, &Token) -> Result<&'a mut Self>;
}

impl Extract for Value {
    fn extract<'a>(&'a mut self, token: &Token) -> Result<&'a mut Value> {
        match *token {
            // on Token::Key extract from Value::Table
            Token::Key(ref s) => match *self {
                Value::Table(ref mut t) =>
                    t.get_mut(&s[..]).ok_or(SEK::HeaderKeyNotFound.into_error()),

                _ => Err(SEK::HeaderPathTypeFailure.into_error()),
            },

            // on Token::Index extract from Value::Array
            Token::Index(i) => match *self {
                Value::Array(ref mut a) => if a.len() < i {
                    Err(SEK::HeaderKeyNotFound.into_error())
                } else {
                    Ok(&mut a[i])
                },

                _ => Err(SEK::HeaderPathTypeFailure.into_error()),
            }
        }
    }
}

pub type EntryResult<T> = RResult<T, ParserError>;

/// Extension trait for top-level toml::Value::Table, will only yield correct results on the
/// top-level Value::Table, but not on intermediate tables.
pub trait Header {
    fn verify(&self) -> Result<()>;
    fn parse(s: &str) -> EntryResult<Value>;
    fn default_header() -> Value;
}

impl Header for Value {

    fn verify(&self) -> Result<()> {
        match *self {
            Value::Table(ref t) => verify_header(&t),
            _ => Err(SE::new(SEK::HeaderTypeFailure, None)),
        }
    }

    fn parse(s: &str) -> EntryResult<Value> {
        use toml::Parser;

        let mut parser = Parser::new(s);
        parser.parse()
            .ok_or(ParserErrorKind::TOMLParserErrors.into())
            .and_then(verify_header_consistency)
            .map(Value::Table)
    }

    fn default_header() -> Value {
        let mut m = BTreeMap::new();

        m.insert(String::from("imag"), {
            let mut imag_map = BTreeMap::<String, Value>::new();

            imag_map.insert(String::from("version"), Value::String(String::from(version!())));
            imag_map.insert(String::from("links"), Value::Array(vec![]));

            Value::Table(imag_map)
        });

        Value::Table(m)
    }

}

pub fn verify_header_consistency(t: Table) -> EntryResult<Table> {
    verify_header(&t)
        .map_err(Box::new)
        .map_err(|e| ParserErrorKind::HeaderInconsistency.into_error_with_cause(e))
        .map(|_| t)
}

fn verify_header(t: &Table) -> Result<()> {
    if !has_main_section(t) {
        Err(SE::from(ParserErrorKind::MissingMainSection.into_error()))
    } else if !has_imag_version_in_main_section(t) {
        Err(SE::from(ParserErrorKind::MissingVersionInfo.into_error()))
    } else if !has_only_tables(t) {
        debug!("Could not verify that it only has tables in its base table");
        Err(SE::from(ParserErrorKind::NonTableInBaseTable.into_error()))
    } else {
        Ok(())
    }
}

fn has_only_tables(t: &Table) -> bool {
    debug!("Verifying that table has only tables");
    t.iter().all(|(_, x)| is_match!(*x, Value::Table(_)))
}

pub fn has_main_section(t: &Table) -> bool {
    t.contains_key("imag") && is_match!(t.get("imag"), Some(&Value::Table(_)))
}

pub fn has_imag_version_in_main_section(t: &Table) -> bool {
    use semver::Version;

    match *t.get("imag").unwrap() {
        Value::Table(ref sec) => {
            sec.get("version")
                .and_then(|v| {
                    match *v {
                        Value::String(ref s) => Some(Version::parse(&s[..]).is_ok()),
                        _                    => Some(false),
                    }
                })
            .unwrap_or(false)
        }
        _ => false,
    }
}


#[cfg(test)]
mod test {
    extern crate env_logger;
    use super::TomlValueExt;
    use super::{tokenize, walk_header};
    use super::Token;

    use std::collections::BTreeMap;

    use toml::Value;

    #[test]
    fn test_walk_header_simple() {
        let tokens = tokenize("a", '.').unwrap();
        assert!(tokens.len() == 1, "1 token was expected, {} were parsed", tokens.len());
        assert!(tokens.iter().next().unwrap() == &Token::Key(String::from("a")),
                "'a' token was expected, {:?} was parsed", tokens.iter().next());

        let mut header = BTreeMap::new();
        header.insert(String::from("a"), Value::Integer(1));

        let mut v_header = Value::Table(header);
        let res = walk_header(&mut v_header, tokens);
        assert_eq!(&mut Value::Integer(1), res.unwrap());
    }

    #[test]
    fn test_walk_header_with_array() {
        let tokens = tokenize("a.0", '.').unwrap();
        assert!(tokens.len() == 2, "2 token was expected, {} were parsed", tokens.len());
        assert!(tokens.iter().next().unwrap() == &Token::Key(String::from("a")),
                "'a' token was expected, {:?} was parsed", tokens.iter().next());

        let mut header = BTreeMap::new();
        let ary = Value::Array(vec![Value::Integer(1)]);
        header.insert(String::from("a"), ary);


        let mut v_header = Value::Table(header);
        let res = walk_header(&mut v_header, tokens);
        assert_eq!(&mut Value::Integer(1), res.unwrap());
    }

    #[test]
    fn test_walk_header_extract_array() {
        let tokens = tokenize("a", '.').unwrap();
        assert!(tokens.len() == 1, "1 token was expected, {} were parsed", tokens.len());
        assert!(tokens.iter().next().unwrap() == &Token::Key(String::from("a")),
                "'a' token was expected, {:?} was parsed", tokens.iter().next());

        let mut header = BTreeMap::new();
        let ary = Value::Array(vec![Value::Integer(1)]);
        header.insert(String::from("a"), ary);

        let mut v_header = Value::Table(header);
        let res = walk_header(&mut v_header, tokens);
        assert_eq!(&mut Value::Array(vec![Value::Integer(1)]), res.unwrap());
    }

    /**
     * Creates a big testing header.
     *
     * JSON equivalent:
     *
     * ```json
     * {
     *      "a": {
     *          "array": [ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9 ]
     *      },
     *      "b": {
     *          "array": [ "string1", "string2", "string3", "string4" ]
     *      },
     *      "c": {
     *          "array": [ 1, "string2", 3, "string4" ]
     *      },
     *      "d": {
     *          "array": [
     *              {
     *                  "d1": 1
     *              },
     *              {
     *                  "d2": 2
     *              },
     *              {
     *                  "d3": 3
     *              },
     *          ],
     *
     *          "something": "else",
     *
     *          "and": {
     *              "something": {
     *                  "totally": "different"
     *              }
     *          }
     *      }
     * }
     * ```
     *
     * The sections "a", "b", "c", "d" are created in the respective helper functions
     * create_header_section_a, create_header_section_b, create_header_section_c and
     * create_header_section_d.
     *
     * These functions can also be used for testing.
     *
     */
    fn create_header() -> Value {
        let a = create_header_section_a();
        let b = create_header_section_b();
        let c = create_header_section_c();
        let d = create_header_section_d();

        let mut header = BTreeMap::new();
        header.insert(String::from("a"), a);
        header.insert(String::from("b"), b);
        header.insert(String::from("c"), c);
        header.insert(String::from("d"), d);

        Value::Table(header)
    }

    fn create_header_section_a() -> Value {
        // 0..10 is exclusive 10
        let a_ary = Value::Array((0..10).map(|x| Value::Integer(x)).collect());

        let mut a_obj = BTreeMap::new();
        a_obj.insert(String::from("array"), a_ary);
        Value::Table(a_obj)
    }

    fn create_header_section_b() -> Value {
        let b_ary = Value::Array((0..9)
                                 .map(|x| Value::String(format!("string{}", x)))
                                 .collect());

        let mut b_obj = BTreeMap::new();
        b_obj.insert(String::from("array"), b_ary);
        Value::Table(b_obj)
    }

    fn create_header_section_c() -> Value {
        let c_ary = Value::Array(
            vec![
                Value::Integer(1),
                Value::String(String::from("string2")),
                Value::Integer(3),
                Value::String(String::from("string4"))
            ]);

        let mut c_obj = BTreeMap::new();
        c_obj.insert(String::from("array"), c_ary);
        Value::Table(c_obj)
    }

    fn create_header_section_d() -> Value {
        let d_ary = Value::Array(
            vec![
                {
                    let mut tab = BTreeMap::new();
                    tab.insert(String::from("d1"), Value::Integer(1));
                    tab
                },
                {
                    let mut tab = BTreeMap::new();
                    tab.insert(String::from("d2"), Value::Integer(2));
                    tab
                },
                {
                    let mut tab = BTreeMap::new();
                    tab.insert(String::from("d3"), Value::Integer(3));
                    tab
                },
            ].into_iter().map(Value::Table).collect());

        let and_obj = Value::Table({
            let mut tab = BTreeMap::new();
            let something_tab = Value::Table({
                let mut tab = BTreeMap::new();
                tab.insert(String::from("totally"), Value::String(String::from("different")));
                tab
            });
            tab.insert(String::from("something"), something_tab);
            tab
        });

        let mut d_obj = BTreeMap::new();
        d_obj.insert(String::from("array"), d_ary);
        d_obj.insert(String::from("something"), Value::String(String::from("else")));
        d_obj.insert(String::from("and"), and_obj);
        Value::Table(d_obj)
    }

    #[test]
    fn test_walk_header_big_a() {
        test_walk_header_extract_section("a", &create_header_section_a());
    }

    #[test]
    fn test_walk_header_big_b() {
        test_walk_header_extract_section("b", &create_header_section_b());
    }

    #[test]
    fn test_walk_header_big_c() {
        test_walk_header_extract_section("c", &create_header_section_c());
    }

    #[test]
    fn test_walk_header_big_d() {
        test_walk_header_extract_section("d", &create_header_section_d());
    }

    fn test_walk_header_extract_section(secname: &str, expected: &Value) {
        let tokens = tokenize(secname, '.').unwrap();
        assert!(tokens.len() == 1, "1 token was expected, {} were parsed", tokens.len());
        assert!(tokens.iter().next().unwrap() == &Token::Key(String::from(secname)),
                "'{}' token was expected, {:?} was parsed", secname, tokens.iter().next());

        let mut header = create_header();
        let res = walk_header(&mut header, tokens);
        assert_eq!(expected, res.unwrap());
    }

    #[test]
    fn test_walk_header_extract_numbers() {
        test_extract_number("a", 0, 0);
        test_extract_number("a", 1, 1);
        test_extract_number("a", 2, 2);
        test_extract_number("a", 3, 3);
        test_extract_number("a", 4, 4);
        test_extract_number("a", 5, 5);
        test_extract_number("a", 6, 6);
        test_extract_number("a", 7, 7);
        test_extract_number("a", 8, 8);
        test_extract_number("a", 9, 9);

        test_extract_number("c", 0, 1);
        test_extract_number("c", 2, 3);
    }

    fn test_extract_number(sec: &str, idx: usize, exp: i64) {
        let tokens = tokenize(&format!("{}.array.{}", sec, idx)[..], '.').unwrap();
        assert!(tokens.len() == 3, "3 token was expected, {} were parsed", tokens.len());
        {
            let mut iter = tokens.iter();

            let tok = iter.next().unwrap();
            let exp = Token::Key(String::from(sec));
            assert!(tok == &exp, "'{}' token was expected, {:?} was parsed", sec, tok);

            let tok = iter.next().unwrap();
            let exp = Token::Key(String::from("array"));
            assert!(tok == &exp, "'array' token was expected, {:?} was parsed", tok);

            let tok = iter.next().unwrap();
            let exp = Token::Index(idx);
            assert!(tok == &exp, "'{}' token was expected, {:?} was parsed", idx, tok);
        }

        let mut header = create_header();
        let res = walk_header(&mut header, tokens);
        assert_eq!(&mut Value::Integer(exp), res.unwrap());
    }

    #[test]
    fn test_header_read() {
        let h = create_header();

        assert!(if let Ok(Some(Value::Table(_)))  = h.read("a") { true } else { false });
        assert!(if let Ok(Some(Value::Array(_)))   = h.read("a.array") { true } else { false });
        assert!(if let Ok(Some(Value::Integer(_))) = h.read("a.array.1") { true } else { false });
        assert!(if let Ok(Some(Value::Integer(_))) = h.read("a.array.9") { true } else { false });

        assert!(if let Ok(Some(Value::Table(_))) = h.read("c") { true } else { false });
        assert!(if let Ok(Some(Value::Array(_)))  = h.read("c.array") { true } else { false });
        assert!(if let Ok(Some(Value::String(_))) = h.read("c.array.1") { true } else { false });
        assert!(if let Ok(None) = h.read("c.array.9") { true } else { false });

        assert!(if let Ok(Some(Value::Integer(_))) = h.read("d.array.0.d1") { true } else { false });
        assert!(if let Ok(None) = h.read("d.array.0.d2") { true } else { false });
        assert!(if let Ok(None) = h.read("d.array.0.d3") { true } else { false });

        assert!(if let Ok(None) = h.read("d.array.1.d1") { true } else { false });
        assert!(if let Ok(Some(Value::Integer(_))) = h.read("d.array.1.d2") { true } else { false });
        assert!(if let Ok(None) = h.read("d.array.1.d3") { true } else { false });

        assert!(if let Ok(None) = h.read("d.array.2.d1") { true } else { false });
        assert!(if let Ok(None) = h.read("d.array.2.d2") { true } else { false });
        assert!(if let Ok(Some(Value::Integer(_))) = h.read("d.array.2.d3") { true } else { false });

        assert!(if let Ok(Some(Value::String(_))) = h.read("d.something") { true } else { false });
        assert!(if let Ok(Some(Value::Table(_))) = h.read("d.and") { true } else { false });
        assert!(if let Ok(Some(Value::Table(_))) = h.read("d.and.something") { true } else { false });
        assert!(if let Ok(Some(Value::String(_))) = h.read("d.and.something.totally") { true } else { false });
    }

    #[test]
    fn test_header_set_override() {
        let _ = env_logger::init();
        let mut h = create_header();

        println!("Testing index 0");
        assert_eq!(h.read("a.array.0").unwrap().unwrap(), Value::Integer(0));

        println!("Altering index 0");
        assert_eq!(h.set("a.array.0", Value::Integer(42)).unwrap().unwrap(), Value::Integer(0));

        println!("Values now: {:?}", h);

        println!("Testing all indexes");
        assert_eq!(h.read("a.array.0").unwrap().unwrap(), Value::Integer(42));
        assert_eq!(h.read("a.array.1").unwrap().unwrap(), Value::Integer(1));
        assert_eq!(h.read("a.array.2").unwrap().unwrap(), Value::Integer(2));
        assert_eq!(h.read("a.array.3").unwrap().unwrap(), Value::Integer(3));
        assert_eq!(h.read("a.array.4").unwrap().unwrap(), Value::Integer(4));
        assert_eq!(h.read("a.array.5").unwrap().unwrap(), Value::Integer(5));
        assert_eq!(h.read("a.array.6").unwrap().unwrap(), Value::Integer(6));
        assert_eq!(h.read("a.array.7").unwrap().unwrap(), Value::Integer(7));
        assert_eq!(h.read("a.array.8").unwrap().unwrap(), Value::Integer(8));
        assert_eq!(h.read("a.array.9").unwrap().unwrap(), Value::Integer(9));
    }

    #[test]
    fn test_header_set_new() {
        let _ = env_logger::init();
        let mut h = create_header();

        assert!(h.read("a.foo").is_ok());
        assert!(h.read("a.foo").unwrap().is_none());

        {
            let v = h.set("a.foo", Value::Integer(42));
            assert!(v.is_ok());
            assert!(v.unwrap().is_none());

            assert!(if let Ok(Some(Value::Table(_))) = h.read("a") { true } else { false });
            assert!(if let Ok(Some(Value::Integer(_))) = h.read("a.foo") { true } else { false });
        }

        {
            let v = h.set("new", Value::Table(BTreeMap::new()));
            assert!(v.is_ok());
            assert!(v.unwrap().is_none());

            let v = h.set("new.subset", Value::Table(BTreeMap::new()));
            assert!(v.is_ok());
            assert!(v.unwrap().is_none());

            let v = h.set("new.subset.dest", Value::Integer(1337));
            assert!(v.is_ok());
            assert!(v.unwrap().is_none());

            assert!(if let Ok(Some(Value::Table(_))) = h.read("new") { true } else { false });
            assert!(if let Ok(Some(Value::Table(_))) = h.read("new.subset") { true } else { false });
            assert!(if let Ok(Some(Value::Integer(_))) = h.read("new.subset.dest") { true } else { false });
        }
    }


    #[test]
    fn test_header_insert_override() {
        let _ = env_logger::init();
        let mut h = create_header();

        println!("Testing index 0");
        assert_eq!(h.read("a.array.0").unwrap().unwrap(), Value::Integer(0));

        println!("Altering index 0");
        assert_eq!(h.insert("a.array.0", Value::Integer(42)).unwrap(), false);
        println!("...should have failed");

        println!("Testing all indexes");
        assert_eq!(h.read("a.array.0").unwrap().unwrap(), Value::Integer(0));
        assert_eq!(h.read("a.array.1").unwrap().unwrap(), Value::Integer(1));
        assert_eq!(h.read("a.array.2").unwrap().unwrap(), Value::Integer(2));
        assert_eq!(h.read("a.array.3").unwrap().unwrap(), Value::Integer(3));
        assert_eq!(h.read("a.array.4").unwrap().unwrap(), Value::Integer(4));
        assert_eq!(h.read("a.array.5").unwrap().unwrap(), Value::Integer(5));
        assert_eq!(h.read("a.array.6").unwrap().unwrap(), Value::Integer(6));
        assert_eq!(h.read("a.array.7").unwrap().unwrap(), Value::Integer(7));
        assert_eq!(h.read("a.array.8").unwrap().unwrap(), Value::Integer(8));
        assert_eq!(h.read("a.array.9").unwrap().unwrap(), Value::Integer(9));
    }

    #[test]
    fn test_header_insert_new() {
        let _ = env_logger::init();
        let mut h = create_header();

        assert!(h.read("a.foo").is_ok());
        assert!(h.read("a.foo").unwrap().is_none());

        {
            let v = h.insert("a.foo", Value::Integer(42));
            assert!(v.is_ok());
            assert_eq!(v.unwrap(), true);

            assert!(if let Ok(Some(Value::Table(_))) = h.read("a") { true } else { false });
            assert!(if let Ok(Some(Value::Integer(_))) = h.read("a.foo") { true } else { false });
        }

        {
            let v = h.insert("new", Value::Table(BTreeMap::new()));
            assert!(v.is_ok());
            assert_eq!(v.unwrap(), true);

            let v = h.insert("new.subset", Value::Table(BTreeMap::new()));
            assert!(v.is_ok());
            assert_eq!(v.unwrap(), true);

            let v = h.insert("new.subset.dest", Value::Integer(1337));
            assert!(v.is_ok());
            assert_eq!(v.unwrap(), true);

            assert!(if let Ok(Some(Value::Table(_))) = h.read("new") { true } else { false });
            assert!(if let Ok(Some(Value::Table(_))) = h.read("new.subset") { true } else { false });
            assert!(if let Ok(Some(Value::Integer(_))) = h.read("new.subset.dest") { true } else { false });
        }
    }

    #[test]
    fn test_header_delete() {
        let _ = env_logger::init();
        let mut h = create_header();

        assert!(if let Ok(Some(Value::Table(_)))   = h.read("a") { true } else { false });
        assert!(if let Ok(Some(Value::Array(_)))   = h.read("a.array") { true } else { false });
        assert!(if let Ok(Some(Value::Integer(_))) = h.read("a.array.1") { true } else { false });
        assert!(if let Ok(Some(Value::Integer(_))) = h.read("a.array.9") { true } else { false });

        assert!(if let Ok(Some(Value::Integer(1))) = h.delete("a.array.1") { true } else { false });
        assert!(if let Ok(Some(Value::Integer(9))) = h.delete("a.array.8") { true } else { false });
        assert!(if let Ok(Some(Value::Array(_)))   = h.delete("a.array") { true } else { false });
        assert!(if let Ok(Some(Value::Table(_)))   = h.delete("a") { true } else { false });

    }

}

