# imag - [imag-pim.org](http://imag-pim.org)

`imag` is a commandline personal information management suite.

**This application is in early development. There are _some_ things that work,
but we do not consider anything stable or usable at this moment. Feel free to
play around anyways.**

[![Build Status](https://travis-ci.org/matthiasbeyer/imag.svg?branch=master)](https://travis-ci.org/matthiasbeyer/imag)
[![Issue Stats](http://www.issuestats.com/github/matthiasbeyer/imag/badge/pr?style=flat-square)](http://www.issuestats.com/github/matthiasbeyer/imag)
[![Issue Stats](http://www.issuestats.com/github/matthiasbeyer/imag/badge/issue?style=flat-square)](http://www.issuestats.com/github/matthiasbeyer/imag)
[![license](https://img.shields.io/github/license/matthiasbeyer/imag.svg?maxAge=2592000?style=flat-square)]()

## What is this / Goal and Functionality

Our (long-term) goal is to

> Create a fast, reliable commandline personal
> information management suite which covers all aspects of personal information
> management, consists of reusable parts and integrates well with known
> commandline tools.

We try to implement as many aspects of personal information management (PIM),
but re-use existing commandline tools.
We do this by tracking/referring to the data the tools create.
A user can now link pieces of data (from different tools), tag this data and
query/search this data using imag.
So `imag` is more like a data-mining helper than an actual PIM tool, but we
implement some of the PIM aspects directly in `imag`.
Parts of PIM (we call them "modules") that are already implemented and basically
working:

* todo (via taskwarrior, we track the tasks one creates in taskwarrior)
* diary
* notes
* bookmarks
* counter (just an example, nothing that usable)

Helper modules that come with `imag` but are not "PIM aspects":

* linking entries
* viewing entries
* tagging entries
* creating misc entries
* creating entries that refer to files/directories

## Building/Running

Here goes how to try `imag` out.

`imag` is a _suite_ of tools and you can build them individually.
All subdirectories prefixed with "`libimag"` are libraries for the respective
binaries.
All subdirectories prefixed with `"imag-"` are binaries and compiling them will
give you a commandline application.

### Building

By now, there are several targets in the Makefile, fulfilling following roles:

* `all` is the default and builds every crate in debug mode.
  To build a single module, call `make <module>`, for example `make imag-store`.
* `release`, as the name implies, builds every module in release mode.
  E.G.: `make imag-store-release` to build "imag-store" in release mode.
* `install` will install all commandline modules to the default installation
  root (see `man cargo-install`).
  To install a single module, run `make <module>-install`,
  E.G.: `make imag-store-install`
* `bin`/`lib` are separate targets for either building all binaries or
  libraries.
* `lib-test` runs `cargo test` for all libraries.
  For testing a single library, E.G.: `make test-libimagstore`.
* `clean` will run `cargo clean` in every crate.
  For cleaning a single crate, use `make imag-store-clean` for example.
* to build _only_ the `imag` binary, use the target `imag-bin`
  (`imag-bin-release` for release build, `imag-bin-clean` for `cargo clean`ing).

### Running

To test out a single module, simply using `cargo run -- <options>` in the
respective directory will do the trick.
But you can also `make <module>` and call the binary on the commandline.
For using it "normally", install the
binaries as described above, as well as the imag-binary:

```
$> make install
```

The installation root of the binaries may not yet be in your $PATH.
To see where this installation root is check out `man cargo-install`.
To change the $PATH in bash:

```bash
$> PATH=$PATH:~/.cargo/bin
$> imag --help
```

To test, simply add `--help` to one of the above commands:

```bash
$> imag counter --help
```

## Staying up-to-date

Despite we have a [official site for imag](http://imag-pim.org), I do not push
updates to this site, yet. Anyways, I post a blog articles about what happened
in the last two weeks every other week.

You can find them
[on my personal blog, tagged "imag"](http://beyermatthias.de/tags/imag.html)

I also post these blog posts
[on reddit](https://www.reddit.com/r/rust/search?q=What%27s+coming+up+in+imag&restrict_sr=on)
and submit them to [this-week-in-rust](https://this-week-in-rust.org/).

From time to time I publish an article about imag which does not focus on some
things that are happening, but rather about something more general.

## Documentation

For detailed information, please read [the documentation](./doc/).
You can either read the Markdown files or compile it to HTML/PDF using
[pandoc](http://pandoc.org).
Developer documentation is also available
[online on github.io](https://matthiasbeyer.github.io/imag/imag_documentation/index.html).

Please note that the documentation is work in progress as well and may be
outdated.

## Please contribute!

We are looking for contributors!

There is always a number of
[complexity/easy tagged issues](https://github.com/matthiasbeyer/imag/issues?q=is%3Aopen+is%3Aissue+label%3Acomplexity%2Feasy)
available in the issue tracker you can start with and we are open to questions!

Feel free to open issues for asking questions, suggesting features or other
things!

Also have a look at [the CONTRIBUTING.md file](./CONTRIBUTING.md)!

## Contact

Have a look at [our website](http://imag-pim.org) where you can find some
information on how to get in touch and so on.

Feel free to join our new IRC channel at freenode: #imag
or our [mailinglist](http://imag-pim.org/mailinglist/).

## License

We chose to distribute this software under terms of GNU LGPLv2.1.

This decision was made to ensure everyone can write applications which use the
imag core functionality which is distributed with the imag source distribution.

