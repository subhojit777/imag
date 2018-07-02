## libimagentryref

This library crate contains functionality to generate _references_ within the
imag store.

A reference is a "pointer" to a file or directory on the filesystem and outside
the store.
It differs from `libimagentrylink`/external linking as
it is designed exclusively for filesystem references, not for URLs.

A reference is created with a unique identifier, like a hash. The implementation
how this hash is calculated can be defined by the user of `libimagentryref`.

So this library helps to resemble something like a _symlink_.

### Usage

Users have to implement the `UniqueRefPathGenerator` trait which should
implement a hashing functionality for pathes.

### Limits

This is _not_ intended to be a version control system or something like that.
We also can not use _real symlinks_ as we need imag-store-objects to be able to
link stuff.

### Usecase

This library offers functionality to refer to content outside of the store.
It can be used to refer to _nearly static stuff_ pretty easily - think of a
Maildir - you add new mails by fetching them, but you mostly do not remove
mails.
If mails get moved, they can be re-found via their hash, because Maildir objects
hardly change. Or because the hash implementation which is used to refer to them
hashes only the `Message-Id` and that does not change.

### Long-term TODO

Not implemented yet:

- [ ] Re-finding of files via their hash.
      This must be implemented with several things in mind
      * The user of the library should be able to provide a way how the
        filesystem is searched. Basically a Functor which yields pathes to
        check based on the original path of the missing file.
        This enables implementations which do only search a certain subset
        of pathes, or does depth-first-search rather than
        breadth-first-search.

### Known problems

The functionality this library provides fails to work when syncing the imag
store between two devices where the data layout is different on each device.

