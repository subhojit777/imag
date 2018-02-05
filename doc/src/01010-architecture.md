# Architecture of the imag code

The imag codebase has a rather simple overall architecture.
In this chapter the types of crates, architecture of an imag module
and the type structure are described.

## Crate types

There are different types of crates in the imag world. A crate is a rust
project.

First of all, there are core crates. These crates provide the very core of imag
and almost all other crates use them:

* libimagstore - The imag store is the abstraction over the filesystem. It
  provides primitives to get, write and manipulate store entries and their
  header information.
* libimagrt - The runtime library, which provides functionality to create a
  store object from libimagstore, helps with configurarion loading and
  commandline argument handling (through the external "clap" crate).
* libimagerror - Error handling library for handling errors the imag way. Used
  in all other crates, even the store itself. It also offers functionality to
  log and trace errors as well as exiting the application, if necessary.
* libimagutil - Utilities.

The next type of imag crates are entry extension libraries. Those provide
extensional functionality for the types from libimagstore. For example, there is
"libimagentrylink" which provides functionality to link two entries in the
store.

The third kind of crate is the one that offers end-user functionality for a imag
domain, for example "libimagtodo" provides functionality to track todos.

And last, but not least, the commandline frontend crates provide the user
interface. These are the kind of crates that are not library crates, but
binaries.

Besides these, there are some other utility crates.

## Architecture of an imag module

With the things from above, a module could have the following architecture:

```
+---------------------------------------------+
|  imag-foo                                   |
+-----------------------------------+---------+
|  libimagfoo                       |         |
+-----------------+-----------------+         |
|                 |                 |         |
| libimagentrybar | libimagentrybaz |         |
|                 |                 |   lib   |
+-----------------+-----------------+         |
|                                   |         |
|  ...                              |         |
|                                   |   imag  |
+-----------------------------------+         |
|                                   |         |
| libimagrt                         |         |
|                                   |  error  |
+-----------------------------------+         |
|                                   |         |
| libimagstore                      |         |
|                                   |         |
+-----------------------------------+---------+
```

The foundation of all imag modules is the store, as one can see in the
visualization from above.
Above the store library there is the libimagrt, which provides the basic runtime
and access to the `Store` object.
Cross-cutting, there is the error library (and possibly
the util library, but we do not care about this one here), which is used through
all levels. The highest level of all imag modules is the commandline interface
on top of the domain library.  In between can be any number of entry extension
libraries, or none if not needed.

Theoretically, the commandline interface crate could be replaced to build a
terminal user interface, graphical user interface or web interface.

## Types

The imag core, hence the libimagstore, libimagrt and libimagerror, provide a set
of types that a user (as in a library writer) should be aware of.

First of all, there is the `Runtime` type which is provided by the libimagrt. It
provides basic access to whether debugging or verbosity is enabled as well as
the most important core object: The `Store`.

The `Store` type is provided by the libimagstore library, the heart of
everything.

When interacting with the store, two types are visible: `FileLockEntry` and
`Entry` whereas the former derefs to the latter, which basically means that the
former wraps the latter.  The `FileLockEntry` is a necessary wrapper for
ensuring that when working concurrently with the store, an entry is only
_borrowed_ once from the store. It also ensures that the object is alive as long
as the store is.

The `Entry` type provides functionality like reading the actual content, its
header and so on. Extensions for its functionality are implemented on this type,
not on the `FileLockEntry`.

The `Entry` provides access to its header, which is a `toml::Value`, where toml
is the toml-rs crate (external project). Convenience functionality is provided
via the `toml-query` crate, which is an external project which was initiated and
extracted from the imag project.

Error types are also important.
All errors in imag projects should be created with `error-chain`.
libimagerror provides functionality to enhance the experience with `Result`
types and general tracing of errors.
