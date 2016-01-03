pub mod markdown {
    use hoedown::renderer::Render;
    use hoedown::Buffer;
    use hoedown::Markdown;

    pub type LinkTitle  = String;
    pub type LinkURL    = String;

    pub struct Link {
        pub title: LinkTitle,
        pub url: LinkURL,
    }

    struct LinkExtractRenderer {
        links : Vec<Link>
    }

    impl LinkExtractRenderer {

        fn new() -> LinkExtractRenderer {
            LinkExtractRenderer {
                links: vec![],
            }
        }

        fn extract(self) -> Vec<Link> {
            self.links
        }

    }

    impl Render for LinkExtractRenderer {

        fn link(&mut self,
                output: &mut Buffer,
                content: &Buffer,
                link: &Buffer,
                title: &Buffer) -> bool {

            let l = String::from(link.to_str().unwrap_or("<<UTF8 Error>>"));
            let t = String::from(title.to_str().unwrap_or("<<UTF8 Error>>"));

            debug!("[Markdown] Push link: '{}' -> '{}'", t, l);
            self.links.push(Link {
                title:  t,
                url:    l,
            });
            true
        }

    }

    pub struct MarkdownParser {
        text: Markdown,
    }

    impl MarkdownParser {

        pub fn new(s: &String) -> MarkdownParser {
            MarkdownParser {
                text: Markdown::new(&s[..])
            }
        }

        pub fn links(&self) -> Vec<Link> {
            let mut renderer = LinkExtractRenderer::new();
            renderer.render(&self.text);
            renderer.extract()
        }

        pub fn to_html(self) -> String {
            use hoedown::renderer::html::Html;
            use hoedown::renderer::html;

            String::from(
                Html::new(html::Flags::empty(), 0)
                    .render(&self.text)
                    .to_str()
                    .unwrap_or("UTF8Error"))
        }

    }

}
