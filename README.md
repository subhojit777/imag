# imag

Imag is a CLI PIM suite you can
integrate in your tools of choice (Editor, MUA, RSS reader, etc etc).

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

One can build all the modules simply by running `make` which defaults to
building all the modules and placing them in the `out/` directory of the project
root.

```
$> make
  ...
$> ls out/
imag-counter  imag-link  imag-notes  imag-store  imag-tag  imag-view
```

Building all the modules may take some time, so alternatively one can build only
a specific module
by runing `$> make $module` where `$module` is one of  the `imag-*` names, such
as `imag-counter`, `imag-link`, etc.

### Running

To run imag, simply call `./out/imag`.
If you include the `out` directory in your `$PATH`, imag is able to find the
other imag executables. Try it out by running:

```bash
$> PATH=$PATH:$(pwd)/out imag --help
```

To test, simply add `--help` to one of the above commands:

```bash
$> PATH=$PATH:$(pwd)/out imag counter --help
```

## Documentation

For detailed information, please read [the documentation](./doc/) (You can
either read the Markdown files or compile it to HTML/PDF using
[pandoc](http://pandoc.org)).

Please note that the documentation is work in progress as well and may be
outdated.

## Contact

You can contact [me](https://github.com/matthiasbeyer) via mail at beyermatthias
dot de if you need to.

Feel free to join our new IRC channel at freenode: #imag.

## License

We chose to distribute this software under terms of GNU LGPLv2.1.

This decision was made to ensure everyone can write applications which use the
imag core functionality which is distributed with the imag source distribution.

