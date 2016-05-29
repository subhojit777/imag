use result::Result;

use hoedown::renderer::Render;
use hoedown::Buffer;
use hoedown::Markdown;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Link {
    pub title: String,
    pub link: String,
}

struct LinkExtractor {
    links: Vec<Link>,
}

impl LinkExtractor {

    pub fn new() -> LinkExtractor {
        LinkExtractor { links: vec![] }
    }

    pub fn links(self) -> Vec<Link> {
        self.links
    }

}

impl Render for LinkExtractor {

    fn link(&mut self,
            _: &mut Buffer,
            content: Option<&Buffer>,
            link: Option<&Buffer>,
            _: Option<&Buffer>)
        -> bool
    {
        let link  = link.and_then(|l| l.to_str().ok()).map(String::from);
        let content = content.and_then(|l| l.to_str().ok()).map(String::from);

        match (link, content) {
            (Some(link), Some(content)) => {
                self.links.push(Link { link: link, title: content });
                false
            },

            (_, _) => {
                false
            },
        }

    }

}

pub fn extract_links(buf: &str) -> Vec<Link> {
    let mut le = LinkExtractor::new();
    le.render(&Markdown::new(buf));
    le.links()
}

#[cfg(test)]
mod test {
    use super::{Link, extract_links};

    #[test]
    fn test_one_link() {
        let testtext = "Some [example text](http://example.com).";

        let exp = Link {
            title: String::from("example text"),
            link:  String::from("http://example.com"),
        };

        let mut links = extract_links(testtext);
        assert_eq!(1, links.len());
        assert_eq!(exp, links.pop().unwrap())
    }

    #[test]
    fn test_two_similar_links() {
        let testtext = r#"
Some [example text](http://example.com).
Some more [example text](http://example.com).
        "#;

        let exp = Link {
            title: String::from("example text"),
            link:  String::from("http://example.com"),
        };

        let mut links = extract_links(&testtext[..]);
        assert_eq!(2, links.len());
        assert_eq!(exp, links.pop().unwrap());
        assert_eq!(exp, links.pop().unwrap());
    }

}

