use regex::Regex;

use std::convert::Into;
use std::convert::From;

use std::option::Option;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KeyValue<K, V> {
    k: K,
    v: V,
}

impl<K, V> KeyValue<K, V> {

    pub fn new(k: K, v: V) -> KeyValue<K, V> {
        KeyValue { k: k, v: v }
    }

    pub fn key(&self) -> &K {
        &self.k
    }

    pub fn value(&self) -> &V {
        &self.v
    }

}

impl<K, V> Into<(K, V)> for KeyValue<K, V> {

    fn into(self) -> (K, V) {
        (self.k, self.v)
    }

}

pub trait IntoKeyValue<K, V> {
    fn into_kv(self) -> Option<KeyValue<K, V>>;
}

impl IntoKeyValue<String, String> for String {

    fn into_kv(self) -> Option<KeyValue<String, String>> {
        let r = "^(?P<KEY>(.*))=((\"(?P<DOUBLE_QVAL>(.*))\")|(\'(?P<SINGLE_QVAL>(.*)))\'|(?P<VAL>[^\'\"](.*)[^\'\"]))$";
        let regex = Regex::new(r).unwrap();
        regex.captures(&self[..]).and_then(|cap| {
            cap.name("KEY")
                .map(|name| {
                    cap.name("SINGLE_QVAL")
                        .or(cap.name("DOUBLE_QVAL"))
                        .or(cap.name("VAL"))
                        .map(|value| KeyValue::new(String::from(name), String::from(value)))
                }).unwrap_or(None)
        })
    }

}

#[cfg(test)]
mod test {
    use super::{KeyValue, IntoKeyValue};

    #[test]
    fn test_single_quoted() {
        let s = String::from("foo='bar'").into_kv().unwrap();
        assert_eq!(KeyValue::new(String::from("foo"), String::from("bar")), s);
    }

    #[test]
    fn test_double_quoted() {
        let s = String::from("foo=\"bar\"").into_kv().unwrap();
        assert_eq!(KeyValue::new(String::from("foo"), String::from("bar")), s);
    }

    #[test]
    fn test_double_and_single_quoted() {
        assert!(String::from("foo=\"bar\'").into_kv().is_none());
    }

    #[test]
    fn test_single_and_double_quoted() {
        assert!(String::from("foo=\'bar\"").into_kv().is_none());
    }

    #[test]
    fn test_not_quoted() {
        let s = String::from("foo=bar").into_kv().unwrap();
        assert_eq!(KeyValue::new(String::from("foo"), String::from("bar")), s);
    }

}


