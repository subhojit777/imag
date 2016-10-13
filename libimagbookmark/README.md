## libimagbookmark

This library crate implements functionality for bookmarks.

It uses `libimagentrylink` to create external links and therefor deduplicates
equivalent external links (`libimagentrylink` deduplicates - you cannot store
two different store entries for `https://imag-pim.org` in the store).

It supports bookmark collections and all basic functionality that one might
need.

