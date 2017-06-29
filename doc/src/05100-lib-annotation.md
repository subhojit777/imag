## libimagannotation

This library provides annotation functionality for entries.

Annotations are normal Store entries, but their header at
`annotation.is_annotation` is set to `true`.

Annotations are linked to an entry (as in `libimagentrylink`).

### Library functionality

The library features two traits: One to extend an `Entry` with annotation
functionality and another one for extending the `Store` with functionality to
get annotations of an entry and all annotations in the store.

