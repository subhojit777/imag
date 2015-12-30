use url::Url;

/**
 * Util: Check wether a String can be parsed as an URL
 */
pub fn is_url(url: &String) -> bool {
    Url::parse(&url[..]).is_ok()
}

