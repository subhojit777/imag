## Link {#sec:modules:link}

The Linking module.

### Description

The linking module `imag-link` is one of the plumbing modules.
It offers the possibility to link entries in the store.

It also offers the functionality to link to external sources. This functionality
_can_ be used to link to external URLs, but the bookmarking module should be
used to do this (see @sec:modules:bookmarks).

The linking module offers functionality to add, remove and list both internal
(store entry to store entry) and external (store entry to URL) links.

#### Internal linking

<!-- internal linking description remains to be written -->

#### External linking

A store entry can only have _one_ external link. Therefor, when you create an
external link, the linking module creates a new entry in the store which links
to this URL. The linking module then links you entry with this new entry by
using an internal link. This way one entry can have multiple external links
attached to it and external links are deduplicated automatically.

### Backends

As this is a plumbing module and only intended to be used with the imag store,
there is no reason to have other backends.

