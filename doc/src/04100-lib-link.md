# liblink {#sec:liblink}

The "liblink" library contains functionality for linking from entries to
internal and external content.

The utilities provided by "liblink" contain functions to

* Get internal links from arbitraty entries
* Access an "EntryHeader" object to
  * get internal links
  * set internal links
  * get "Entry" objects for links from a header

## Linking internal content with liblink {#sec:liblink:internal}

As one entry MAY contain zero or more interal links in the header section
"imag.links", the "liblink" library provides functionality to

* get the link from an EntryHeader object
* set the links in a EntryHeader object
* query for a specific link in a EntryHeader object
  * by regex
  * by module name
  * by filename

as well as functionality to get "Entry" objects for each entry in the Header.

## Linking external content with liblink {#sec:liblink:external}

As one "EntryHeader" MUST NOT contain more than one link to external content (as
defined in @sec:thestore:linking:external, the following scheme for linking to
external content MUST BE provided by "liblink":

* Generate a "link" entry in the store
  * with store path starting with "/link/"
  * where the header field "imag.content.uri" MUST BE set
  * with optional content which can be stored in the section of the "liblink"
    module section (section name "link", as defined by
    @sec:thestore:fileformat:header:module).
* Get an external link by store path (looking up the store path entry and
  getting the link to the external content from it)

