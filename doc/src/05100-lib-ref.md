## libimagref

This library crate contains functionality to generate _references_ within the
imag store.

It can be used to create references to other files on the filesystem (reachable
via a filesystem path). It differs from `libimagentrylink`/external linking as
it is designed exclusively for filesystem references, not for URLs.

A reference can have several properties, for example can a reference track the
content of a filesystem path by hashing the content with a hashsum (SHA1) and
one can check whether a file was changed by that.
As files can get big (think of `debian.iso`) _partial hashing_ is supported
(think of "hash the first 2048 bytes of a file).

The library contains functionality to re-find a moved file automatically by
checking the content hash which was stored before.

Permission changes can be tracked as well.

So this library helps to resemble something like a _symlink_.

### Limits

Please understand that this is _not_ intended to be a version control system or
something like that.
We also can not use _real symlinks_ as we need imag-store-objects to be able to
link stuff.

### Usecase

This library offers functionality to refer to content outside of the store.
It can be used to refer to _nearly static stuff_ pretty easily - think of a
Maildir - you add new mails by fetching them, but you mostly do not remove mails
and if you do you end up with a "null pointer" in the store, which can then be
handled properly.

As this library supports custom hashes (you don't have to hash the full file,
you can also parse the file and hash only _some_ content) this is pretty
flexible.
For example if you want to implement a imag module which tracks a certain kind
of files which constantly change... but the first 5 lines do never change
after the file is created - you can write a custom hasher that only uses the
first 5 lines for the hash.

### Internals

Internally, in the store, the file gets created under
`/ref/<hash of the path to the file to refer to>`.
If the content of the file is hashed, we can still re-find the file via the
content hash (which is stored in the header of the store entry).

The reference object can, after the path was re-found, be updated.

