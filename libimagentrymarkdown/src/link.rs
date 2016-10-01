//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015, 2016 Matthias Beyer <mail@beyermatthias.de> and contributors
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; version
// 2.1 of the License.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
//

use error::MarkdownErrorKind as MEK;
use result::Result;

use hoedown::renderer::Render;
use hoedown::Buffer;
use hoedown::Markdown;
use url::Url;

use libimagerror::into::IntoError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Link {
    pub title: String,
    pub link: String,
}

impl Link {

    /// Translate a `Link` into a `UrlLink`
    fn into_urllink(self) -> Result<UrlLink> {
        Url::parse(&self.link[..])
            .map(move |link| UrlLink { title: self.title, link: link, })
            .map_err(Box::new)
            .map_err(|e| MEK::LinkParsingError.into_error_with_cause(e))
    }

}

pub struct UrlLink {
    pub title: String,
    pub link: Url,
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

    #[test]
    fn test_two_links() {
        let testtext = r#"
Some [example text](http://example.com).
Some more [foo](http://example.com/foo).
        "#;

        let exp1 = Link {
            title: String::from("example text"),
            link:  String::from("http://example.com"),
        };

        let exp2 = Link {
            title: String::from("foo"),
            link:  String::from("http://example.com/foo"),
        };

        let mut links = extract_links(&testtext[..]);
        assert_eq!(2, links.len());
        assert_eq!(exp2, links.pop().unwrap());
        assert_eq!(exp1, links.pop().unwrap());
    }

}

