# Contributing to imag

So you want to contribute to imag! Thank you, that's awesome!

If you already have something in mind, go ahead with [the prerequisites
section](#prerequisites). If you don't know what you could do, start here.

All contributors agree to the
[developer certificate of origin](#developer-certificate-of-origin)
by contributing to imag.

## Without Github

If you do not want to use github for your contribution, this is completely okay.
Feel free to contact [us via our mailinglist](http://imag-pim.org/mailinglist/)
via mail, feel also free to submit patches via mail (use `git format-patch` and
`git send-email`, always add a cover letter to describe your submission).

Also ensure that each commit has
[a "Signed-off-by: " line](https://stackoverflow.com/questions/1962094/what-is-the-sign-off-feature-in-git-for).
By adding that line, you agree to our
[developer certificate of origin](#developer-certificate-of-origin).

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

Also, if you found bugs or outdated stuff in our documentation, feel free to
file issues about them or even better: Write a pull request to fix them!

## Prerequisites

* cargo and rust compiler in current version (stable)
* Ruby and Bundler if you want to write a Ruby module.

Dependencies are listed in the
[default.nix file](http://git.imag-pim.org/imag/tree/default.nix),
though you do not have to have `nix` installed to build imag.

`make` can be helpful to automate the build process.

Note that this software is targeted towards commandline linux users and we do
not aim to be portable to Windows or Mac OSX (though I wouldn't mind merging
patches for OS X compatibility).

* If you want to build the documentation (you don't have to)
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
your history before merging!

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

1. It is written in Rust or Ruby
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

## More information about the structure of this project

Here goes some notes on how this project is structured.

### Issue- and PR-Labels

Our labels are color coded as well as "namespaced". The color groups labels
exactly as the prefix does. The prefix is the first component which is seperated
from the others by `"/`". See below:

| Label                     | Description                       | search                                     |
| ---                       | ---                               | ---                                        |
| complexity/easy           | Easy to do                        | [search][search-complexity/easy]           |
| complexity/high           | Not so easy to do                 | [search][search-complexity/high]           |
| complexity/medium         | Relatively easy                   | [search][search-complexity/medium]         |
| kind/bug                  | Bug                               | [search][search-kind/bug]                  |
| kind/doc                  | Documentation related             | [search][search-kind/doc]                  |
| kind/enhancement          | Enhancement                       | [search][search-kind/enhancement]          |
| kind/feature              | Feature                           | [search][search-kind/feature]              |
| kind/hotfix               | Hotfix                            | [search][search-kind/hotfix]               |
| kind/infrastructure       | Infrastructure code               | [search][search-kind/infrastructure]       |
| kind/invalid              | Not valid Issue/PR                | [search][search-kind/invalid]              |
| kind/nicetohave           | Would be a nice thing             | [search][search-kind/nicetohave]           |
| kind/refactor             | Refactor codebase                 | [search][search-kind/refactor]             |
| meta/assigned             | Is assigned                       | [search][search-meta/assigned]             |
| meta/blocked              | Blocked by other Issue/PR         | [search][search-meta/blocked]              |
| meta/blocker              | Blocks other Issue/PR             | [search][search-meta/blocker]              |
| meta/decision-pending     | Not clear what to do              | [search][search-meta/decision-pending]     |
| meta/dependencies         | Dependency-related                | [search][search-meta/dependencies]         |
| meta/doc                  | Documentation related             | [search][search-meta/doc]                  |
| meta/importance/high      | Very Important                    | [search][search-meta/importance/high]      |
| meta/importance/low       | Not so important                  | [search][search-meta/importance/low]       |
| meta/importance/medium    | Rather important                  | [search][search-meta/importance/medium]    |
| meta/on-hold              | Do not work on this!              | [search][search-meta/on-hold]              |
| meta/ready                | Ready for review/merge            | [search][search-meta/ready]                |
| meta/reopen-later         | Reopen closed issue/pr later      | [search][search-meta/reopen-later]         |
| meta/WIP                  | Work in Progress                  | [search][search-meta/WIP]                  |
| nochange/duplicate        | Duplicated                        | [search][search-nochange/duplicate]        |
| nochange/question         | Question                          | [search][search-nochange/question]         |
| nochange/rfc              | Request for comments              | [search][search-nochange/rfc]              |
| nochange/wontfix          | Won't fix this issue              | [search][search-nochange/wontfix]          |
| part/bin/imag-counter     | Targets binary: imag-counter      | [search][search-part/bin/imag-counter]     |
| part/bin/imag-link        | Targets binary: imag-link         | [search][search-part/bin/imag-link]        |
| part/bin/imag-store       | Targets binary: imag-store        | [search][search-part/bin/imag-store]       |
| part/bin/imag-tag         | Targets binary: imag-tag          | [search][search-part/bin/imag-tag]         |
| part/bin/imag-view        | Targets binary: imag-view         | [search][search-part/bin/imag-view]        |
| part/interface            | Changes the interface             | [search][search-part/interface]            |
| part/lib/imagcounter      | Targets library: imagcounter      | [search][search-part/lib/imagcounter]      |
| part/lib/imagentryfilter  | Targets library: imagentryfilter  | [search][search-part/lib/imagentryfilter]  |
| part/lib/imagentrylink    | Targets library: imagentrylink    | [search][search-part/lib/imagentrylink]    |
| part/lib/imagentrylist    | Targets library: imagentrylist    | [search][search-part/lib/imagentrylist]    |
| part/lib/imagentrymarkup  | Targets library: imagentrymarkup  | [search][search-part/lib/imagentrymarkup]  |
| part/lib/imagentryprinter | Targets library: imagentryprinter | [search][search-part/lib/imagentryprinter] |
| part/lib/imagentrytag     | Targets library: imagentrytag     | [search][search-part/lib/imagentrytag]     |
| part/lib/imagentryview    | Targets library: imagentryview    | [search][search-part/lib/imagentryview]    |
| part/lib/imagnotes        | Targets library: imagnotes        | [search][search-part/lib/imagnotes]        |
| part/lib/imagrt           | Targets library: imagrt           | [search][search-part/lib/imagrt]           |
| part/lib/imagstore        | Targets library: imagstore        | [search][search-part/lib/imagstore]        |
| part/lib/imagutil         | Targets library:                  | [search][search-part/lib/imagutil]         |
| part/_new_binary          | Introduces new binary             | [search][search-part/_new_binary]          |
| part/_new_library         | Introduces new library            | [search][search-part/_new_library]         |
| test/change               | Changes a test                    | [search][search-test/change]               |
| test/missing              | Test missing                      | [search][search-test/missing]              |
| test/new                  | New test                          | [search][search-test/new]                  |

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

## FAQ

_to be written_

[search-complexity/easy]: https://github.com/matthiasbeyer/imag/labels/
[search-complexity/high]: https://github.com/matthiasbeyer/imag/labels/complexity%2Fhigh
[search-complexity/medium]: https://github.com/matthiasbeyer/imag/labels/complexity%2Fmedium
[search-kind/bug]: https://github.com/matthiasbeyer/imag/labels/kind%2Fbug
[search-kind/doc]: https://github.com/matthiasbeyer/imag/labels/kind%2Fdoc
[search-kind/enhancement]: https://github.com/matthiasbeyer/imag/labels/kind%2Fenhancement
[search-kind/feature]: https://github.com/matthiasbeyer/imag/labels/kind%2Ffeature
[search-kind/hotfix]: https://github.com/matthiasbeyer/imag/labels/kind%2Fhotfix
[search-kind/infrastructure]: https://github.com/matthiasbeyer/imag/labels/kind%2Finfrastructure
[search-kind/invalid]: https://github.com/matthiasbeyer/imag/labels/kind%2Finvalid
[search-kind/nicetohave]: https://github.com/matthiasbeyer/imag/labels/kind%2Fnicetohave
[search-kind/refactor]: https://github.com/matthiasbeyer/imag/labels/kind%2Frefactor
[search-meta/assigned]: https://github.com/matthiasbeyer/imag/labels/meta%2Fassigned
[search-meta/blocked]: https://github.com/matthiasbeyer/imag/labels/meta%2Fblocked
[search-meta/blocker]: https://github.com/matthiasbeyer/imag/labels/meta%2Fblocker
[search-meta/decision-pending]: https://github.com/matthiasbeyer/imag/labels/meta%2Fdecision-pending
[search-meta/dependencies]: https://github.com/matthiasbeyer/imag/labels/meta%2Fdependencies
[search-meta/doc]: https://github.com/matthiasbeyer/imag/labels/meta%2Fdoc
[search-meta/importance/high]: https://github.com/matthiasbeyer/imag/labels/meta%2Fimportance%2Fhigh
[search-meta/importance/low]: https://github.com/matthiasbeyer/imag/labels/meta%2Fimportance%2Flow
[search-meta/importance/medium]: https://github.com/matthiasbeyer/imag/labels/meta%2Fimportance%2Fmedium
[search-meta/on-hold]: https://github.com/matthiasbeyer/imag/labels/meta%2Fon-hold
[search-meta/ready]: https://github.com/matthiasbeyer/imag/labels/meta%2Fready
[search-meta/reopen-later]: https://github.com/matthiasbeyer/imag/labels/meta%2Freopen-later
[search-meta/WIP]: https://github.com/matthiasbeyer/imag/labels/meta%2FWIP
[search-nochange/duplicate]: https://github.com/matthiasbeyer/imag/labels/nochange%2Fduplicate
[search-nochange/question]: https://github.com/matthiasbeyer/imag/labels/nochange%2Fquestion
[search-nochange/rfc]: https://github.com/matthiasbeyer/imag/labels/nochange%2Frfc
[search-nochange/wontfix]: https://github.com/matthiasbeyer/imag/labels/nochange%2Fwontfix
[search-part/bin/imag-counter]: https://github.com/matthiasbeyer/imag/labels/part%2Fbin%2Fimag-counter
[search-part/bin/imag-link]: https://github.com/matthiasbeyer/imag/labels/part%2Fbin%2Fimag-link
[search-part/bin/imag-store]: https://github.com/matthiasbeyer/imag/labels/part%2Fbin%2Fimag-store
[search-part/bin/imag-tag]: https://github.com/matthiasbeyer/imag/labels/part%2Fbin%2Fimag-tag
[search-part/bin/imag-view]: https://github.com/matthiasbeyer/imag/labels/part%2Fbin%2Fimag-view
[search-part/interface]: https://github.com/matthiasbeyer/imag/labels/part%2F_interface
[search-part/lib/imagcounter]: https://github.com/matthiasbeyer/imag/labels/part%2Flib%2Fimagcounter
[search-part/lib/imagentryfilter]: https://github.com/matthiasbeyer/imag/labels/part%2Flib%2Fimagentryfilter
[search-part/lib/imagentrylink]: https://github.com/matthiasbeyer/imag/labels/part%2Flib%2Fimagentrylink
[search-part/lib/imagentrylist]: https://github.com/matthiasbeyer/imag/labels/part%2Flib%2Fimagentrylist
[search-part/lib/imagentrymarkup]: https://github.com/matthiasbeyer/imag/labels/part%2Flib%2Fimagentrymarkup
[search-part/lib/imagentryprinter]: https://github.com/matthiasbeyer/imag/labels/part%2Flib%2Fimagentryprinter
[search-part/lib/imagentrytag]: https://github.com/matthiasbeyer/imag/labels/part%2Flib%2Fimagentrytag
[search-part/lib/imagentryview]: https://github.com/matthiasbeyer/imag/labels/part%2Flib%2Fimagentryview
[search-part/lib/imagnotes]: https://github.com/matthiasbeyer/imag/labels/part%2Flib%2Fimagnotes
[search-part/lib/imagrt]: https://github.com/matthiasbeyer/imag/labels/part%2Flib%2Fimagrt
[search-part/lib/imagstore]: https://github.com/matthiasbeyer/imag/labels/part%2Flib%2Fimagstore
[search-part/lib/imagutil]: https://github.com/matthiasbeyer/imag/labels/part%2Flib%2Fimagutil
[search-part/_new_binary]: https://github.com/matthiasbeyer/imag/labels/part%2F_new_binary
[search-part/_new_library]: https://github.com/matthiasbeyer/imag/labels/part%2F_new_library
[search-test/change]: https://github.com/matthiasbeyer/imag/labels/test%2Fchange
[search-test/missing]: https://github.com/matthiasbeyer/imag/labels/test%2Fmissing
[search-test/new]: https://github.com/matthiasbeyer/imag/labels/test%2Fnew
