# Contributing to imag

Prerequisites:

* cargo and rust compiler in current version (stable)

In particular, we seek the following types of contributions:

* Ideas: What is a PIM module you want to see implemented or (even better) could
  you implement yourself?
* Testing: Feel free to test `imag`. Note that we have no release yet.
* Documentation: We want to have 100% documentation coverage. If you find
  something undocumented or wrongly documented, don't hesitate to fix it and
  send a PR!

# Commit guidelines, PR guidelines:

Please don't refer to issues or PRs from inside a commit message, if possible.
Make sure your PR does not contain "Fixup" commits when publishing it, but feel
free to push "Fixup" commits in the review process. We will ask you to clean
your history before merging!

Make sure to prefix your commits with `"doc: "` if you change the document. Do
not change document and code in one commit, always seperate them.

# Merging tools which use the imag core functionality into this repo

If you're writing an application or module for imag, feel free to propose
integrating it into the imag core distribution, if it fulfills the following
requirements:

1. It is written in Rust
1. It has a commandline interface which is the main interface to the module.
1. It is licensed under the terms of GNU LGPLv2.1

(If your tool does not fulfill these requirements, I won't merge it into the
imag core distribution.)

# Code of Conduct

We use the same
[code of conduct as the rust community does](https://www.rust-lang.org/conduct.html).

# Contact

Feel free to reach out via mail.

# FAQ

_to be written_

