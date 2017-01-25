# imag-ruby

A Ruby gem for scripting imag modules.

## How does this work?

Well, as we have some problems with lifetimes here, we have a fairly complex
codebase in this crate.

### The Problem

The Problem is, that `libimagstore::store::FileLockEntry<'a>` has a lifetime. If
we would wrap this object into a ruru wrapper and pass to the Ruby code, we
couldn't guarantee anymore that the lifetime holds.

The problem is simple, you see...

### The solution?

Never pass anything to the Ruby code.

Yes, exactly. The Ruby code only sees 'handles'. It never actually gets the
`Store` object either.
We move the `Store` Object into a `Cache` object (actually, the Ruby code could
have multiple `Store` objects to work with this way) and return a `StoreHandle`
to the Ruby code (which is a UUID underneath).

Also, the Ruby code never actually touches a `FileLockEntry` - it only gets a
Handle for each `FileLockEntry` - which is a tuple of the `StoreHandle` and the
`libimagstore::storeid::StoreId` for the Entry.

Each operation on a `FileLockEntry` is then wrapped by this very library. Each
time `FileLockEntry` is touched, this library fetches the appropriate `Store`
object from the static `Cache`, then fetches the `FileLockEntry` object from it,
does the operation and then drops the object (which implies that the actual
`FileLockEntry` is `update()`d!).

### The Hell?

Yes, I know this is a lot of overhead. But what are we talking about here? This
is Ruby code we're talking about here, so speed is not our concern.

You could argue this is a hell of complexity introduced in this library and yes
it is.
If there are bugs (and I bet there are) they would be complex as hell.
But that's it... if you have a better approach, please file a PR.

## Tests?

We have tests Ruby scripts in `./test`, they are not executed by travis-ci, as
we need Ruby `2.3.0` for this and travis has `2.2.0` as latest version.
But I hope we get it in travis soonish.

## Ruby gem?

This crate will contain both the Rust bindings for imag using `ruru` and a bunch
of wrapper code for the actual `imag` gem.

### Why another layer of indirection?

As "ruru" does not yet support modules (which is sad btw) we would end up with
functions for all the things.

E.G.: `imag_runtime_setup()` instead of `Imag::Runtime::setup()`

I want to add a Ruby gem to wrap these things.

So basically a piece of Ruby which uses the Rust code to build
`imag` as a gem which then exports a fine module system.

### The module system:

```text
Imag (Module)
  EntryContent (Class (inherits from String))
  EntryHeader (Class)
  FileLockEntryHandle (Class)
  StoreHandle (Class)
  StoreId (Class)
```

`libimagentrytag` and the other `libimagentry*` libraries will be pulled into
this library to support more advanced operations with the `FileLockEntryHandle`
type.

