use regex::Regex;

pub fn split_into_key_value(s: String) -> Option<(String, String)> {
    let r = "^(?P<KEY>(.*))=((\"(?P<DOUBLE_QVAL>(.*))\")|(\'(?P<SINGLE_QVAL>(.*)))\'|(?P<VAL>[^\'\"](.*)[^\'\"]))$";
    let regex = Regex::new(r).unwrap();
    regex.captures(&s[..]).and_then(|cap| {
        cap.name("KEY")
            .map(|name| {
                cap.name("SINGLE_QVAL")
                    .or(cap.name("DOUBLE_QVAL"))
                    .or(cap.name("VAL"))
                    .map(|value| (String::from(name), String::from(value)))
            }).unwrap_or(None)
    })
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


