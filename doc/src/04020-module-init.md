## Init {#sec:modules:init}

This is the only `imag-*` command which does _not_ set up a runtime and check
whether the store is available. This command can be used to set up a imag store.

It also puts a default configuration in the right place and initializes a git
repository, if there is a `git` command in `$PATH` (via calling git on the
commandline, not via `libgit2` or some other library).

