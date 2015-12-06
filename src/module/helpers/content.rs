mod markdown {
    use pulldown_cmark::Parser;
    use pulldown_cmark::Event;
    use pulldown_cmark::Tag;

    pub struct MarkdownParser<'a> {
        text: &'a String
    }

    impl<'a> MarkdownParser<'a> {

        pub fn new(s: &'a String) -> MarkdownParser {
            MarkdownParser {
                text: s
            }
        }

        pub fn links(&self) -> Vec<String> {
            Parser::new(&self.text[..])
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

        pub fn codeblocks(&self) -> Vec<String> {
            Parser::new(&self.text[..])
                .filter_map(|e| {
                    match e {
                        Event::Start(t) => Some(t),
                        Event::End(t)   => Some(t),
                        _               => None
                    }
                })
                .filter_map(|tag| {
                    match tag {
                        Tag::CodeBlock(text)    => Some(text),
                        _                       => None
                    }
                })
                .map(|text| {
                    text.into_owned()
                }).collect::<Vec<String>>()
        }

    }

}
