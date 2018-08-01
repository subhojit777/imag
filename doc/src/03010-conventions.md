# Conventions, best practices

This section explains conventions used in the imag codebase. It is mainly
focused on developers, but a user may read it for getting to know how imag
works.

Lets work our way up from the store and how to extend it to the commandline user
interface.

## Versioning

All imag crates are versioned with the same version number until we reach some
`"1.0.0"` version.
This means that all imag tools are only tested for compatibility with libraries
and such if their version numbers match.
It might not be possible to import one imag library in version 0.3.0 and another
one in 0.4.0 and make them work together.
It also means that if new tools are introduced into the imag codebase, they
might start with their first version not at 0.1.0 but at something like 0.5.0.

## Store and Entry functionality

A `Entry` does not offer much functionality by itself. So its the job of
libraries to _extend_ its functionality. This should never be done by wrapping
the `Entry` type itself but by providing and implementing an extension trait on
it.

Same goes for extending the `Store` type: never wrap it, always provide an
extension trait for it.

These two rules ensure that the type does not lose any functionality from a
wrapping. `Deref` could do that, but not over muliple levels, so extension
traits it is. It also most likely results in functions inside the extension
trait which all return a `Result<_, _>`.

## Libraries

In the next few sections, conventions and best practices for writing a imag
library are written down.

A developer of imag should read this carefully, a user may skip this section or
cross-read it for better understanding of the imag project.

### Library naming

Libraries which provide functionality for entries or the store but no
domain-functionality should be named "libimagentrything" whereas "thing" stands for
what the library provides.

All domain libraries should be prefixed with "libimag". 


### Library scope

A library should never introduce utility functionality which could be useful for
other libraries as well. If there is no such functionality available, the
"libimagutil" or "libimagentryutil" might be a place where such a function
would go to.

If a library has to introduce free functions in its public interface, one should
think hard whether this is really necessary.


### Library error types/kinds

Libraries must use "error-chain" to create error types and kinds.
Most likely, a library needs some kinds for wrapping the errors from underlying
libraries, such as the store itself.

A library must _never_ introduce multiple error types, but is free to introduce
as many error kinds as required.

### Libraries with commandline frontends

Libraries with commandline frontends provide end-user functionality.
They are called "domain" libraries.
Normally,
they depend on one or more "libimagentrything" libraries. They should be named
"libimagthing", though. For example: "libimagdiary", "libimagtimetrack" or
"libimagwiki", whereas the commandline frontends would be "imag-diary",
"imag-timetrack" and "imag-wiki", respectively.

If such a library needs to depend on another "libimagthing", for example if
"libimagdiary" needs to depend on "libimagnote", one should think about this and
whether the functionality could be outsourced to a more general
"libimagentrything".


### Library testing

All libraries should be tested as much as possible. Sometimes it may not be
possible without a lot of effort, but still: more tests = better!

## Commandline tools

The commandline tools are the CLI-frontends for their respective libraries.
So `libimagdiary` has a CLI frontend `imag-diary`.

Those CLI frontends use functionality from `libimagrt` to build a 
commandline interface which is consistent with the rest of the ecosystem.

Commandline interfaces should receive store IDs as positional arguments.
Commandline interfaces should also provide a flag "-I" (that's a big i) which
marks that the store IDs shall be read from stdin and are not passed via the
commandline.


### IO

There are minor restrictions how imag tools should do IO. A good rule of thumb
is (but most certainly only applicable when programming an imag tool in Rust):
use `libimagrt` to do IO of any kind.

For more information, or if not using Rust as programming language: the
documentation of `libimagrt` describes how IO should happen (which output
stream to use, how input should be done).

