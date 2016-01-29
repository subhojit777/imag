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
        let key = {
            let r = "^(?P<KEY>([^=]*))=(.*)$";
            let r = Regex::new(r).unwrap();
            r.captures(&self[..])
                .and_then(|caps| caps.name("KEY"))
        };

        let value = {
            let r = "(.*)=(\"(?P<QVALUE>([^\"]*))\"|(?P<VALUE>(.*)))$";
            let r = Regex::new(r).unwrap();
            r.captures(&self[..])
                .map(|caps| {
                    caps.name("VALUE")
                        .or(caps.name("QVALUE"))
                        .unwrap_or("")
                })
        };

        key.and_then(|k| {
            value.and_then(|v| {
                Some(KeyValue::new(String::from(k), String::from(v)))
            })
        })
    }

}

#[cfg(test)]
mod test {
    use super::{KeyValue, IntoKeyValue};

    #[test]
    fn test_single_quoted() {
        let s = String::from("foo='bar'").into_kv().unwrap();
        assert_eq!(KeyValue::new(String::from("foo"), String::from("\'bar\'")), s);
    }

    #[test]
    fn test_double_quoted() {
        let s = String::from("foo=\"bar\"").into_kv().unwrap();
        assert_eq!(KeyValue::new(String::from("foo"), String::from("bar")), s);
    }

    #[test]
    fn test_double_and_single_quoted() {
        let s = String::from("foo=\"bar\'").into_kv().unwrap();
        assert_eq!(KeyValue::new(String::from("foo"), String::from("\"bar\'")), s);
    }

    #[test]
    fn test_single_and_double_quoted() {
        let s = String::from("foo=\'bar\"").into_kv().unwrap();
        assert_eq!(KeyValue::new(String::from("foo"), String::from("\'bar\"")), s);
    }

    #[test]
    fn test_not_quoted() {
        let s = String::from("foo=bar").into_kv().unwrap();
        assert_eq!(KeyValue::new(String::from("foo"), String::from("bar")), s);
    }

}


