# imag-ruby

A Ruby gem for scripting imag modules.

This crate contains both the Rust bindings for imag using `ruru` and a bunch of
wrapper code for the actual `imag` gem.

## Why another layer of indirection?

As "ruru" does not yet support modules, which is sad btw, we would end up with
functions for all the things.

E.G.: `imag_runtime_setup()` instead of `IMAG::Runtime::setup()`

I want to add a Ruby gem to wrap these things.
So basically a piece of ruby which uses `imag.rb` (the Rust gem) to build
`imag` as a gem which then exports a fine module system.

## Ideas for module system:

```text
IMAG (Module)
  Runtime (Module)
    Runtime (Class)
  Store (Module)
    Store (Class)
    Entry (Class)
    StoreId (Class)
  Util (Module, Ruby-only I guess)
```

I would name the types the same as in the Rust codebase, to avoid confusion.
Only exception would be the `Entry` class, which would be a `FileLockEntry`
underneath and if we adapt `libimagentrytag` and the other `libimagentry*`
libraries, we would extend this type.

