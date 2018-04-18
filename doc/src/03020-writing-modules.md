# Writing an imag module

So you want to write a module for imag.
That's nice.

This guide helps you getting started.
It also can help you understanding how imag modules work, so even if you do
not want to write a full new module, but extend or alter one, this guide may
help you.


## Data layout

First, you have to think about what data you want to store.
What functionality do you want to provide and what data that creates.

In this example, we're writing a module that stores numbers. We're writing the
appropriate library for that as well as a commandline frontend.


## libimagnumberstorage

We're writing a `libimagnumberstorage` which provides the core functionality
of our module: Storing numbers.

That library can then be used by other library authors and by the commandline
interface implementation.

### Setup

So what do we need to do to write this library:

1. Create a new "lib" crate.
   Because we're writing a "domain" library, we're doing this in the
   `lib/domain` subdirectory:
   `cd lib/domain; cargo new --lib libimagnumberstorage`.
1. After creating the library, we have to add the new library to the
   `/Cargo.toml` field and add the missing metadata in the new
   `/lib/domain/libimagnumberstorage/Cargo.toml` file.

That was the setup part.
Now we can implement our functionality.
For that, we need to _extend_ two types from `libimagstore`, so we have our
first dependency here.

### Dependencies to other libraries

3. Put `libimagstore` as a dependency in the
   `/lib/domain/libimagnumberstorage/Cargo.toml` file.
   By using
   `libimagstore = { version = "0.8.0", path = "../../../lib/core/libimagstore" }`
   we automatically get all the goodness of Cargo, so that releases
   automagically work as expected, but when developing locally, the local
   version of `libimagstore` is used.
   Of course, the `version` has to be the latest released version.
4. For error handling, we also need to import `libimagerror`.
5. For easy header-editing, we import `toml` and `toml-query`.
6. For error-type creating, we import `error-chain`.

### Interface

7. Then, we have to _extend_ two types:
    1. `libimagstore::store::Store` has to be extended so we can implement a
       CRUD interface for our special entries.
    1. `libimagstore::store::Entry` has to be extended so we can get our
       stored numbers in a convenient way.

Our interface should roughly look like this:

```
store.get_stored_number("5") -> Result<FileLockEntry, _>
store.store_number("5")      -> Result<FileLockEntry, _>
store.delete_number("5")     -> Result<(), _>
```

You notice that the `Store` returns `FileLockEntry` objects rather than
`Entry` objects.
And that's ok. A `FileLockEntry` is a `Entry`, but ensures that we are the
only ones editing that entry.
So, we have to implement our number-storing-interface on `Entry` as well:

```
entry.get_number() -> Result<usize>
entry.set_number(usize) -> Result<()>
```

All those "extensions" are implemented as traits which are then implemented
for `Store` and `Entry`.

Normally, we create new files for that, as well as for the error types we need:

* `/lib/domain/libimagnumberstorage/src/store.rs`
* `/lib/domain/libimagnumberstorage/src/entry.rs`
* `/lib/domain/libimagnumberstorage/src/error.rs`

where `store.rs` contains a trait `NumberStorage` and `entry.rs` contains a
trait `NumberEntry`.
`error.rs` contains the invocation of the `error_chain!{}` macro.
Error types from `libimagstore` and others are linked in.

