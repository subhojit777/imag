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

Yes, imag is a rather ambitious project as it tries to reimplement functionality
for several "personal information management aspects". It is a hobby project,
keep that in mind. We try to use standards like vcard, icalendar and others
wherever possible.

Have a look at [the documentation](./doc/) for some more words on this.


## Building/Running

Here is how to try `imag` out.

`imag` is a _suite/collection_ of tools (like git, for example) and you can
build them individually.
All subdirectories prefixed with "`libimag"` are libraries.
All subdirectories prefixed with `"imag-"` are binaries and compiling them will
give you a commandline application.

### Building

We use `cargo` for building all crates in this repository.
Make sure to use a recent `cargo`, at least one with workspace support.
Building all crates works with `cargo build --all`, building individual crates
by `cd`ing to their directory and calling `cargo build`.

For building all commandline applications:

```bash
find bin -maxdepth 3 -name Cargo.toml -exec cargo build --manifest-path {} \;
```

For building only the core functionality

```bash
find bin/core -maxdepth 3 -name Cargo.toml -exec cargo build --manifest-path {} \;
```

### Running

After you build the module you want to play with, you can simply call the binary
itself with the `--help` flag, to get some help what the module is capable of.

If you installed the module, you can either call `imag-<modulename>` (if the
install-directory is in your `$PATH`), or install the `imag` binary to call `imag
<modulename>` (also if everything is in your `$PATH`).


## Example usage

As imag is a big and complex project, we cannot show all tools of the suite
here. But to give you some idea, here's an example:

```bash
# Lets initialize imag
imag init

# Recursively import vcf files
imag contact import /home/user/contacts

# Create a contact (vcf) in the private collection
imag contact create --file /home/user/contacts/private

# Add a diary entry
imag diary -p private create

# Uh, I forgot something in a diary entry, select one and edit it
# use the `fzf` tool here (not a part of imag) to select from the IDs
imag diary -p private list | fzf -m | imag edit -I

# Link a contact to the diary entry
imag link diary/private/2018/01/01/00:00:00 contact/bc222298-casf-40a4-bda1-50aa980a68c9

# Annotate a contact with some notes
imag annotate add contact/bc222298-casf-40a4-bda1-50aa980a68c9 contact-notes

# Write down some notes named "pineapple"
imag notes create "pineapple"

# Where was that contact again?
imag grep Eva
# Okay, we need to add some imag-internal notes to that contact
imag grep Eva -l | imag edit -I

# Now save our work
imag git add . # "imag-git" simply calls git in the imag store
imag git commit -m 'Commit message'
```


## Staying up-to-date

We have a [official website for imag](https://imag-pim.org), where I post
[release notes](http://imag-pim.org/releases/) and monthly(ish) updates what's
happening in the source tree ([RSS here](https://imag-pim.org/index.xml)).

We also have a [mailinglist](https://imag-pim.org/mailinglist/) where I post
updates and where discussion and questions are encouraged.


## Documentation

We have some documentation in [the ./doc subtree](./doc/)
which can be compiled to PDF or a website.
It might not be up to date, though.
Developer documentation for the last release is available
[on docs.rs](https://docs.rs/releases/search?query=imag).


## Please contribute!

We are looking for contributors!
Feel free to open issues for asking questions, suggesting features or other
things!

Also have a look at [the CONTRIBUTING.md file](./CONTRIBUTING.md)!


## Contact

Feel free to join our new IRC channel at freenode: #imag
or our [mailinglist](https://imag-pim.org/mailinglist/).


## License

We chose to distribute this software under terms of GNU LGPLv2.1.


