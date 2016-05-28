pub trait Parse : Sized {

    fn parse(s: &str) -> Option<Self>;

}

pub fn time_parse_regex() -> &'static str {
    r#"(?P<Y>\d{4})-(?P<M>\d{2})-(?P<D>\d{2})(T(?P<h>\d{2})(:(?P<m>\d{2})(:(?P<s>\d{2}))?)?)?$"#
}

