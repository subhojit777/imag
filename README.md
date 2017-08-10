# imag - [imag-pim.org](https://imag-pim.org)

`imag` is a commandline personal information management suite.

[![Build Status](https://travis-ci.org/matthiasbeyer/imag.svg?branch=master)](https://travis-ci.org/matthiasbeyer/imag)
[![Issue Stats](http://www.issuestats.com/github/matthiasbeyer/imag/badge/pr?style=flat-square)](http://www.issuestats.com/github/matthiasbeyer/imag)
[![Issue Stats](http://www.issuestats.com/github/matthiasbeyer/imag/badge/issue?style=flat-square)](http://www.issuestats.com/github/matthiasbeyer/imag)
[![license](https://img.shields.io/github/license/matthiasbeyer/imag.svg?maxAge=2592000?style=flat-square)]()
[![Tokei](https://tokei.rs/b1/github/matthiasbeyer/imag)](https://github.com/matthiasbeyer/imag)

**This application is in early development. There are _some_ things that work,
but we do not consider anything stable or usable at this moment. Feel free to
play around anyways.**

## Goal / What is imag?

Our (long-term) goal is to

> Create a fast, reliable commandline personal
> information management suite which covers all aspects of personal information
> management, consists of reusable parts and integrates well with known
> commandline tools.

Yes, imag is a rather ambitous project as it tries to reimplement functionality for several "personal information management aspects". It is a hobby project, keep that in mind. We try to use standards like vcard and icalendar whereever possible.

imag consists of _modules_ (e.g. `imag-notes`, `imag-tag`, `imag-view`), where
each module covers one PIM aspect.

## Building/Running

Here is how to try `imag` out.

`imag` is a _suite/collection_ of tools (like git, for example) and you can
build them individually.
All subdirectories prefixed with "`libimag"` are libraries.
All subdirectories prefixed with `"imag-"` are binaries and compiling them will
give you a commandline application.

### Building

We use `make` to automate the build process.
Make sure to _not_ include some `-j 8` arguments, this will _not_ work as you
might think, as the rust compiler automatically parallelizes stuff and if you `make -j 8`, you end up with a pretty high load!

There are several targets for each of the sub-crates in the Makefile:

| Target         | Multi | Purpose                                  | Example              |
| :---           | ----- | :---                                     | :---                 |
| all            |       | Build everything, debug mode             | `make all`           |
| bin            |       | Build all binaries, debug mode           | `make bin`           |
| lib            |       | Build all libraries, debug mode          | `make lib`           |
| lib-test       |       | Test all libraries                       | `make lib-test`      |
| imag-bin       |       | Build only the `imag` binary, debug mode | `make imag-bin`      |
| clean          |       | Remove build artifacts                   | `make clean`         |
| update         |       | Run `cargo update`                       | `make update`        |
| check-outdated |       | Run `cargo outdated`                     | `make check-outdated`|
| check          | *     | Run `cargo check`                        | `make check`         |
| install        | *     | Build everything, release mode, install  | `make install`       |
| release        | *     | Build everything, release mode           | `make release`       |

The `Multi` targets are callable for each sub-crate. For example you can call
`make imag-store-check` to run `cargo check` on the `imag-store` crate.

### Running

After you build the module you want to play with, you can simply call the binary
itself with the `--help` flag, to get some help what the module is capable of.

If you installed the module, you can either call `imag-<modulename>` (if the
install-directory is in your `$PATH`), or install the `imag` binary to call `imag
<modulename>` (also if everything is in your `$PATH`).

## Staying up-to-date

We have a [official website for imag](https://imag-pim.org), where I post
[release notes](http://imag-pim.org/releases/) and monthly(ish) updates what's happening in the source tree.
There is no RSS feed, though.

We also have a [mailinglist](https://imag-pim.org/mailinglist/) where I post
updates and where discussion and questions are encouraged.

## Documentation

This is a hobby project, so sometimes things are not optimal and might go
unrecognized and slip through. Feel free to open issues about things you notice!

Though, we have some documentation in [the ./doc subtree](./doc/)
which can be compiled to PDF or a website.
These docs are not published anywhere and are not even integrated into our CI,
so it might be broken (though it's unlikely).
Developer documentation is also available
[online on github.io](https://matthiasbeyer.github.io/imag/imag_documentation/index.html)
and [on docs.rs](https://docs.rs/releases/search?query=imag), though they might
be a bit outdated.

## Please contribute!

We are looking for contributors!

Feel free to open issues for asking questions, suggesting features or other
things!

Also have a look at [the CONTRIBUTING.md file](./CONTRIBUTING.md)!

## Contact

Have a look at [our website](https://imag-pim.org) where you can find some
information on how to get in touch and so on.

Feel free to join our new IRC channel at freenode: #imag
or our [mailinglist](https://imag-pim.org/mailinglist/).

## License

We chose to distribute this software under terms of GNU LGPLv2.1.

