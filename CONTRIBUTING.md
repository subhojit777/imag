# Contributing to imag

So you want to contribute to imag! Thank you, that's awesome of you!

If you already have something in mind, go ahead with [the prerequisites
section](#prerequisites). If you don't know what you could do, start here.

## Finding an issue

Finding an issue is simple: We have
[a special label in our issues section](https://github.com/matthiasbeyer/imag/issues?q=is%3Aissue+is%3Aopen+label%3Acomplexity%2Feasy)
for easy-to-solve issues. You can start there, don't hesitate to ask questions
if you do not understand the issue comment!

Also, if you found bugs or outdated stuff in our documentation, feel free to
file issues about them or even better: Write a pull request to fix them!

## Prerequisites

* cargo and rust compiler in current version (stable)

That's it so far, you don't need no additional dependencies. Note that this
software is targeted towards commandline linux users and we do not aim to be
portable to Windows or Mac OSX (though I wouldn't mind merging patches for OS X
compatibility).

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
not change document and code in one commit, always seperate them.

We do not follow some official rust styleguide for our codebase, but we try to
write minimal and readable code. 100 characters per line, as few lines as
possible, avoid noise in the codebase, ... you get it.

Not all of your commits have to be buildable. But your PR has to be.

## PR guidelines:

We'd like to have one PR per module change. This means you _should_ only change
one imag module in one commit (library plus belonging binary is okay). As this
is not always possible, we do not enforce this, though we might ask you to split
your PR into two smaller ones.

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

Feel free to reach out via mail.

## FAQ

_to be written_

