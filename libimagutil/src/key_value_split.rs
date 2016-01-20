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
    use super::split_into_key_value;

    #[test]
    fn test_single_quoted() {
        let s = String::from("foo='bar'");
        assert_eq!(Some((String::from("foo"), String::from("bar"))), split_into_key_value(s));
    }

    #[test]
    fn test_double_quoted() {
        let s = String::from("foo=\"bar\"");
        assert_eq!(Some((String::from("foo"), String::from("bar"))), split_into_key_value(s));
    }

    #[test]
    fn test_double_and_single_quoted() {
        let s = String::from("foo=\"bar\'");
        assert!(split_into_key_value(s).is_none());
    }

    #[test]
    fn test_single_and_double_quoted() {
        let s = String::from("foo=\'bar\"");
        assert!(split_into_key_value(s).is_none());
    }

    #[test]
    fn test_not_quoted() {
        let s = String::from("foo=bar");
        assert_eq!(Some((String::from("foo"), String::from("bar"))), split_into_key_value(s));
    }

}


