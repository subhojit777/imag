pub trait Parse : Sized {

    fn parse(s: &str) -> Option<Self>;

}

