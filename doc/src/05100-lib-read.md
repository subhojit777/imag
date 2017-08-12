## libimagread

This library is for the plumbing command `imag-read`.

It extends the `Runtime` object and adds a `write_store_to(writer)` function (amongst others). After calling this function, the calling program cannot continue to do things, so this consumes the `Runtime` object and the calling program is expected to exit with the returned error code.

The calling program is expected to _not_ print anything to stdout before or after calling this function.

This library is intended for use with the `imag-read` command only.

