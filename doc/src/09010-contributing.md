# Contributing to imag {#sec:contributing}

So you want to contribute to imag! Thank you, that's awesome!

All contributors agree to the
[developer certificate of origin](#developer-certificate-of-origin)
by contributing to imag.


Feel free to contact [us via our mailinglist](http://imag-pim.org/mailinglist/)
and/or submit patches via mail (use `git format-patch` and
`git send-email`, always add a cover letter to describe your submission).

Also ensure that each commit submitted via email has
[a "Signed-off-by: " line](https://stackoverflow.com/questions/1962094/what-is-the-sign-off-feature-in-git-for).
By adding that line, you agree to our
[developer certificate of origin](#developer-certificate-of-origin).
If you do not add the "Signed-off-by: " line, I reserve the right to kindly
reject your patch.

Make sure to test-compile your patchset and, if available, run tests.


## Finding an issue


## Prerequisites

The prerequisites are simple: `cargo` and `rustc` in current version (stable)
or newer (we do not use nighly features though).

Build dependencies for building are listed in the
[default.nix file](http://git.imag-pim.org/imag/tree/default.nix),
though you do not have to have the `nix` package manager installed to build
imag.
Everything else will be done by `cargo`.

Note that this software is targeted towards commandline linux users and we do
not aim to be portable to Windows or Mac OSX (though I wouldn't mind merging
patches for OS X compatibility).

If you want to build the documentation (you don't have to) you'll need:

* pandoc
* pandoc-citeproc
* texlive
* lmodern (font package)
* (gnu) make

All dependencies are installable with the nix package manager by using a
`nix-shell`, if you have the nix package manager installed on your system.


## Commit guidelines

Make sure your patchset does not contain "Fixup" commits when publishing it, but feel
free to send  "Fixup" commits in the review process. 
If squashing fails I will come back to you.

We do not follow some official Rust styleguide for our codebase, but we try to
write minimal and readable code. 100 characters per line, avoid noise in the
codebase, ... you get it.


## Code of Conduct

We use the same
[code of conduct as the rust community does](https://www.rust-lang.org/conduct.html).

Basically: Be kind, encourage others to ask questions - you are encouraged to
ask questions as well!


## Developer Certificate of Origin

```
Developer Certificate of Origin
Version 1.1

Copyright (C) 2004, 2006 The Linux Foundation and its contributors.
660 York Street, Suite 102,
San Francisco, CA 94110 USA

Everyone is permitted to copy and distribute verbatim copies of this
license document, but changing it is not allowed.


Developer's Certificate of Origin 1.1

By making a contribution to this project, I certify that:

(a) The contribution was created in whole or in part by me and I
    have the right to submit it under the open source license
    indicated in the file; or

(b) The contribution is based upon previous work that, to the best
    of my knowledge, is covered under an appropriate open source
    license and I have the right under that license to submit that
    work with modifications, whether created in whole or in part
    by me, under the same open source license (unless I am
    permitted to submit under a different license), as indicated
    in the file; or

(c) The contribution was provided directly to me by some other
    person who certified (a), (b) or (c) and I have not modified
    it.

(d) I understand and agree that this project and the contribution
    are public and that a record of the contribution (including all
    personal information I submit with it, including my sign-off) is
    maintained indefinitely and may be redistributed consistent with
    this project or the open source license(s) involved.
```

