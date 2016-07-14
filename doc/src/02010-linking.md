## Linking from an store entry {#sec:thestore:linking}

As described in @sec:intro:problem the purpose of imag is to _link_ content
together. The following section describes, from a technical view, how this is
done in imag.

There are two ways of linking in imag. You can either link internally or
externally. The following sections describe the differences.

### Linking to internal content {#sec:thestore:linking:internal}

Internal links are links between store entries themselfes. This means that one
store entry can link to another. Actually, links are not pointers but rather
tries between entries, meaning that an link is not directed, but always a
two-way pointer.

How linking works from the user interface is described in @sec:modules:link.

### Linking to external content {#sec:thestore:linking:external}

Linking to external content means linking to files or directories which do not
live inside the store itself but outside of it.

Each store entry can store _one link to external content at most_.

External linking should not be used from the user interface but rather the
`ref` feature (@sec:modules:ref) should be used.
@sec:modules:ref describes why that is.

