## Linking from an store entry {#sec:thestore:linking}

### Linking to internal content {#sec:thestore:linking:internal}

### Linking to external content {#sec:thestore:linking:external}

Each Entry can store _one link to external content at most_.

This link is stored in the header field "imag.content.uri"
(@sec:thestore:fileformat:header:imag).
A key "imag.content.file" COULD be used for a local mirror of the content which
is referenced by "imag.content.uri".

