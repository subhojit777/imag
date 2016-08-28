# imag - [imag-pim.org](http://imag-pim.org)

Imag is a CLI PIM suite you can
integrate in your tools of choice (Editor, MUA, RSS reader, etc etc).

[![Build Status](https://travis-ci.org/matthiasbeyer/imag.svg?branch=master)](https://travis-ci.org/matthiasbeyer/imag)
[![Issue Stats](http://www.issuestats.com/github/matthiasbeyer/imag/badge/pr?style=flat-square)](http://www.issuestats.com/github/matthiasbeyer/imag)
[![Issue Stats](http://www.issuestats.com/github/matthiasbeyer/imag/badge/issue?style=flat-square)](http://www.issuestats.com/github/matthiasbeyer/imag)

## Please contribute!

We are looking work contributors!

There is always a number of
[complexity/easy tagged issues](https://github.com/matthiasbeyer/imag/issues?q=is%3Aopen+is%3Aissue+label%3Acomplexity%2Feasy)
available in the issue tracker you can start with and we are open to questions!

Feel free to open issues for asking questions, suggesting features or other
things!

Also have a look at [the CONTRIBUTING.md file](./CONTRIBUTING.md)!

## Goal

Our goal is to

> Create a fast, reliable, forwards/backwards compatible commandline personal
> information management suite which covers all aspects of personal information
> management, consists of reusable parts and integrates well with known
> commandline tools.

We try to accomplish these requirements:

* "fast": We use the awesome, fast and safe programming language "Rust"
* "reliable": We try to test every aspect of our software. Our build process
  ensures that the build breaks whenever a library interface changes and the
  modules which use the library are not updated.
* "forwards/backwards compatible:" Our (plain text) on-disk data format and
  storage library both ensure that incompatibilities are captured and resolved
  ([using](https://crates.io/crates/semver) [semver](https://semver.org))
* "commandline": We ensure that everything can be done by commandline calls, for
  some modules there might be a curses-like UI, but there are no graphical
  clients and there never will be any within this codebase. We use
  [clap](https://crates.io/crates/clap) for commandline-interface building and
  we try to keep the interface easy and consistent between modules.
* "personal": We store everything as plain text in a store inside the users
  `$HOME` directory. There will be a version-control (most surely `git`) hook
  integrated to sync between several machines. There are no multi-user features
  included or planned at the time of writing.
* "information management": We want to give the user the possibility to put
  every single information about their personal lives into the store and we try
  hard to provide a sane interface to query and retrieve data from this
  database.
* "covers all the aspects of personal information management": We want to
  provide modules for:
  * contact management
  * calendar
  * diary
  * notes
  * personal wiki
  * news (rss)
  * passwords
  * images
  * music
  * movies
  * personal project management
  * podcast management
  * ledger
  * mail
  * bibliography management
  * ... and many, many more.
* "constists of reusable parts": Every functionality is implemented as library.
  The binaries we ship are just commandline-interace-to-library-interface
  translators
* "integrates well with known commandline tools": We do not re-invent the wheel.
  **We do not implement "yet another password manager", but use
  [the standard unix password manager](https://www.passwordstore.org/), do not
  implement a news reader, but use [newsbeuter](http://www.newsbeuter.org/),
  do not reimplement a mail reader, etc etc.**
  We do not copy images, movies or other data to the store but "link" them into
  the store, so you can use imag tools to query and access this data, but still
  live with your beloved commandline apps. We do not want to duplicate work but
  reuse as much as possible.
  You don't like one of the applications we use (for example `pass` as password
  manager)? Sure, feel free to submit patches so the user is able to switch the
  used tool, as long as it doesn't break the workflow. We will happily merge
  them!

## Current state of development

**This application is in _really_ early development.**

We have implemented the very core of the system, though some more utility work
is to be done.
We have the store working, a hooks API and some default hooks are in
development.
Basic features like tagging and linking entries is possible as well as viewing
entries.
Some small things are implemented, like a note-taking module, a basic diary
module, a counter module and a bookmark module.
These modules contain basic features and are subject to change.
More modules are about to be implemented.

Though, the very core of the system is stable and nothing prevents _you_ from
contributing and implementing a module.

## Building/Running

Here goes how to try imag out.

### Building

By now, there are several targets in the Makefile, fulfilling following roles:
* `all` Is the default and builds every crate in debug mode. This is the same as
  traversing every directory yourself and calling `cargo build` in it.
  To build a single crate, call `make <crate>`, for example
  `make imag-store`
* `release`, as the name implies, builds every crate in release mode. Following
  the example above, to build `imag-store` in release mode, call
  `make imag-store-release`.
* `install` will install all binary crates to the default installation root (see
  `man cargo-install`). To install a single module, run `make <module>-install`,
  again, for example: `make imag-store-install`
* `bin`/`lib` are separate targets for either building all binaries or
  libraries.
* `lib-test` runs `cargo test` for all libraries. For testing a single library,
  run `make test-libimagstore` for example.
* `clean` will run `cargo clean` in every crate. Again, for cleaning a single
  crate, use `make imag-store-clean` for example.

**There is currently no target for the `imag` binary itself. Please
build/install it yourself using `cargo build --manifest-path ./bin/Cargo.toml`**

### Running

To test out a single module, simply using `cargo run -- <options>` in the
respective directory will do the trick. For using it "normally", install the
binaries as described above, as well as the imag-binary:
```
$> make install
$> cargo install --path ./bin
```
The installation root of the binaries (a.k.a. where they are installed to), may
not yet be in your $PATH. To see, where this installation root is, check out
`man cargo-install`. To change the $PATH in bash:

```bash
$> PATH=$PATH:~/.cargo/bin
$> imag --help
```

To test, simply add `--help` to one of the above commands:

```bash
$> imag counter --help
```

Please note that $PATH will be reset in a new shell. To make these changes
permanent, see the User Guide of your shell.

## Documentation

For detailed information, please read [the documentation](./doc/) (You can
either read the Markdown files or compile it to HTML/PDF using
[pandoc](http://pandoc.org)).
Developer documentation is also available
[online on github.io](https://matthiasbeyer.github.io/imag/imag_documentation/index.html).

Please note that the documentation is work in progress as well and may be
outdated.

## Contact

Have a look at [our website](http://imag-pim.org) where you can find some
information on how to get in touch and so on.

Feel free to join our new IRC channel at freenode: #imag
or our [mailinglist](http://imag-pim.org/mailinglist/).

## License

We chose to distribute this software under terms of GNU LGPLv2.1.

This decision was made to ensure everyone can write applications which use the
imag core functionality which is distributed with the imag source distribution.

