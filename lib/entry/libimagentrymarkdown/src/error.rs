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

use url::Url;

use libimagstore::storeid::StoreId;

error_chain! {
    types {
        MarkdownError, MarkdownErrorKind, ResultExt, Result;
    }

    links {
        StoreError(::libimagstore::error::StoreError, ::libimagstore::error::StoreErrorKind);
        LinkError(::libimagentrylink::error::LinkError, ::libimagentrylink::error::LinkErrorKind);
        RefError(::libimagentryref::error::RefError, ::libimagentryref::error::RefErrorKind);
    }

    foreign_links {
        UrlParserError(::url::ParseError);
    }

    errors {
        MarkdownRenderError {
            description("Markdown render error")
            display("Markdown render error")
        }

        LinkParsingError    {
            description("Link parsing error")
            display("Link parsing error")
        }

        StoreGetError(id: StoreId) {
            description("Failed to get entry from store")
            display("Failed to get entry '{}' from store", id)
        }

        UndecidableLinkType(s: String) {
            description("Failed to qualify link type")
            display("The Type of the link '{}' cannot be recognized", s)
        }

        UrlProcessingError(u: Url) {
            description("Failed to properly processing URL")
            display("The URL '{:?}' could not be processed properly", u)
        }
    }
}

