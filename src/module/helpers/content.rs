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
            self.extract_tag(|tag| {
                match tag {
                    Tag::Link(url, _)   => Some(url.into_owned()),
                    _                   => None
                }
            })
        }

        pub fn codeblocks(&self) -> Vec<String> {
            self.extract_tag(|tag| {
                match tag {
                    Tag::CodeBlock(text)    => Some(text.into_owned()),
                    _                       => None
                }
            })
        }

        fn extract_tag<F>(&self, f: F) -> Vec<String>
            where F: FnMut(Tag) -> Option<String>
        {
            Parser::new(&self.text[..])
                .filter_map(|e| {
                    match e {
                        Event::Start(t) | Event::End(t) => Some(t),
                        _                               => None
                    }
                })
                .filter_map(f)
                .collect::<Vec<String>>()
        }

    }

}
