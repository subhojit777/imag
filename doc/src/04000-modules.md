# Modules {#sec:modules}

A module is a functionality of the program.
There is a huge list of modules available in the imag core distribution.

From a naming perspective, we do not differ between low-level and high-level
modules. Some of the modules shipped with imag cover core functionality such as
linking, tagging or references to files outside of the store or even the store
interface itself (which by the way shouldn't be used by the end-user at all).
Others cover things like diary, notes, wiki or bookmarks.

We try really hard to offer a consistent commandline user interface over all of
these modules.

The following sections describe each module in detail, including its purpose and
its provided backends.

A backend is simply an external tool imag might be able to use.
For example, the `imag-todo` module offers a `taskwarrior` interface, so imag
itself does not cover anything which has to do with todo management, but lets
you continue using `taskwarrior` for that (which does a really good job).
So what does the `imag-todo` module do?
Well, it offers you ways to track tasks created in `taskwarrior` and putting
files which can be used as references to tasks then.
For example, if you create a task in `taskwarrior`, you end up with an UUID for
this task.
imag stores this UUID in a store entry and you are now able to `imag-link` this
file with other files in the store.
This way you can link `taskwarrior` tasks with other data (of course,
`imag-todo` offers some more commands, for searching tasks and so on).

But what if you do not like `taskwarrior`?
That's what backends are for.
The goal of imag is to provide backends for not just one tool which implements a
PIM aspect, but for many.
So you can change the configuration for `imag-todo` to not use `taskwarrior` but
some other todo tool.

(This is all hypothetical by now because these things are not yet implemented.
Anyhow, we aim for exactly what is described above)

