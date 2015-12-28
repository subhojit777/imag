use url::Url;

pub fn is_url(url: &String) -> bool {
    Url::parse(&url[..]).is_ok()
}

