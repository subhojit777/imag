## libimagwrite

This library is for the plumbing command `imag-write`.

It extends the `Runtime` object and adds a `read_store_from(reader)` function (amongst others). After calling this function, the calling program cannot continue to do things, so this consumes the `Runtime` object and the calling program is expected to exit with the returned error code.

The calling program is expected to _not_ print or read anything to/from stdout/stdin before or after calling this function.

This library is intended for use with the `imag-write` command only.


