# Documentation of the idea

This subdirectory contains the documentation of the basic idea behind ``imag''.
It is written in Markdown and compiled to both PDF and HTML via pandoc.

# Contributing to this paper

First, the paper is not build by travis-ci. This means if there are errors, they
will be detected by me only if I build the paper. I know this is not optimal,
but as documented in [70](https://github.com/matthiasbeyer/imag/pull/70),
building the paper in travis would slow down the travis-ci machines too much.

So if you want to contribute I would like you to build the paper, if you can.
The dependencies you need are listed in the `default.nix` file (you don't need
either nix nor nixos, all of these packages should be available in all major
distros). Make sure you use pandoc `1.10+`.

Contributing to this paper is done via normal pull requests, RFC-Like.

That's all to it so far.

