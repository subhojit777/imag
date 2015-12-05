
pub fn extract_links(s: &String) -> Vec<String> {
    use pulldown_cmark::Parser;
    use pulldown_cmark::Event;
    use pulldown_cmark::Tag;

    Parser::new(&s[..])
        .filter_map(|e| {
            match e {
                Event::Start(t) => Some(t),
                Event::End(t)   => Some(t),
                _               => None
            }
        })
        .filter_map(|tag| {
            match tag {
                Tag::Link(url, text) => Some((url, text)),
                _               => None
            }
        })
        .map(|(url, text)| {
            text.into_owned()
        }).collect::<Vec<String>>()
}

