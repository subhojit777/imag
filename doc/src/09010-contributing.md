# Contributing to imag

So you want to contribute to imag! Thank you, that's awesome!

All contributors agree to the
[developer certificate of origin](#developer-certificate-of-origin)
by contributing to imag.

If you already have something in mind, go ahead with [the prerequisites
section](#prerequisites). If you don't know what you could do, start here.

## Without Github

If you do not want to use github for your contribution, this is completely okay.
Feel free to contact [us via our mailinglist](http://imag-pim.org/mailinglist/)
via mail, feel also free to submit patches via mail (use `git format-patch` and
`git send-email`, always add a cover letter to describe your submission).

Also ensure that each commit has
[a "Signed-off-by: " line](https://stackoverflow.com/questions/1962094/what-is-the-sign-off-feature-in-git-for).
By adding that line, you agree to our
[developer certificate of origin](#developer-certificate-of-origin).
If you do not add the "Signed-off-by: " line, I cannot take your patch, sorry.

Once _I am_ okay with your patchset, I will
submit it as PR in the github repository, so more people can review it and CI
can test it (the mailinglist is not yet used as much as github). I might come
back to you if something broke in CI or someone has a suggestion how to improve
your PR. I will keep you as author of the commits.

The following sections describe the way how to contribute with github.

## Finding an issue

Finding an issue is simple: We have
[a special label in our issues section](https://github.com/matthiasbeyer/imag/issues?q=is%3Aissue+is%3Aopen+label%3Acomplexity%2Feasy)
for easy-to-solve issues. You can start there, don't hesitate to ask questions
if you do not understand the issue comment!

Also, if you've found bugs or outdated stuff in our documentation, feel free to
file issues about them or even better: Write a pull request to fix them!

## Prerequisites

* cargo and rust compiler in current version (stable)

Dependencies are listed in the
[default.nix file](http://git.imag-pim.org/imag/tree/default.nix),
though you do not have to have `nix` installed to build imag.

`make` can be helpful to automate the build process.

Note that this software is targeted towards commandline linux users and we do
not aim to be portable to Windows or Mac OSX (though I wouldn't mind merging
patches for OS X compatibility).

If you want to build the documentation (you don't have to) you'll need:

* pandoc
* pandoc-citeproc
* texlive
* lmodern (font package)
* make

All dependencies are installable with the nix package manager by using a
`nix-shell`, if you have the nix package manager installed on your system.

## Commit guidelines

Please don't refer to issues or PRs from inside a commit message, if possible.
Make sure your PR does not contain "Fixup" commits when publishing it, but feel
free to push "Fixup" commits in the review process. We will ask you to clean
your history before merging! If you're submitting via patch-mail, I will do the fixup squashing myself.

Make sure to prefix your commits with `"doc: "` if you change the document. Do
not change document and code in one commit, always separate them.

We do not follow some official Rust styleguide for our codebase, but we try to
write minimal and readable code. 100 characters per line, as few lines as
possible, avoid noise in the codebase, ... you get it.

Not all of your commits have to be buildable. But your PR has to be.

## PR guidelines

We'd like to have one PR per module change. This means you _should_ only change
one imag module in one commit or PR (library plus belonging binary is okay).
As this is not always possible, we do not enforce this, though we might ask you
to split your commits/PR into two smaller ones.

Use feature branches. If you could name them "<module name>/<what you do>",
for example "libimagstore/add-debugging-calls", that would be awesome.

You are welcome to publish your PR as soon as there is one commit in your
branch. This gives us the possibility to review whether your ideas go into a
nice direction or whether there are issues with your approach and we can report
them to you rather quickly. Rewriting a whole PR is not satisfactory and we'd
like to make your contribution process enjoyable.

# Merging tools which use the imag core functionality into this repo

If you're writing an application or module for imag, feel free to propose
integrating it into the imag core distribution, if it fulfills the following
requirements:

1. It is written in Rust
1. It has a commandline interface which is the main interface to the module
   OR it is a utility library for creating new kinds of functionality within the
   imag core.
1. It is licensed under the terms of GNU LGPLv2.1 OR all of your contributors
   approve a commit which changes the license of your codebase to GNU LGPLv2.1
   (The word "approve" in this sentence is to be defined).

(If your tool does not fulfill these requirements, I won't merge it into the
imag core distribution.)

## Code of Conduct

We use the same
[code of conduct as the rust community does](https://www.rust-lang.org/conduct.html).

Basically: Be kind, encourage others to ask questions - you are encouraged to
ask questions as well!

## Contact

Feel free to reach out via mail/[mailinglist](http://imag-pim.org/mailinglist/)
or [IRC](irc://irc.freenode.net/#imag).

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

