# imag

Imag is a CLI PIM suite with a nice API-ish commandline interface, so you can
integrate it in your tools of coice (Editor, MUA, RSS reader, etc etc).

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

We have implemented the very
core of the system: the store library. There's also some progress on utility
libraries for linking entries, tagging and the hook system of the store is in
progress as well.
There is also one commandline application: "imag-store" (the "store" subcommand)
available by now, but this is meant for developers and  debugging purposes as it
provides direct core-level store access.

Though, the very core of the system is stable and nothing prevents _you_ from
contributing and implementing a module, though some convenience is not yet
provided (as the libraries are work-in-progress).

## Documentation

For detailed information, please read [the documentation](./doc/) (You can
either read the Markdown files or compile it to HTML/PDF using
[pandoc](http://pandoc.org)).

## License

We chose to distribute this software under terms of GNU LGPLv2.1.

This dicision was made to ensure everyone can write applications which use the
imag core functionality which is distributed with the imag source distribution.

