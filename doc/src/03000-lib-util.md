# libutil {#sec:libutil}

<!--
    Might not get this big, but its here for DRYness
-->

The utility library of the project contains utility functionality which is
used by all other libraries and/or binaries.

It is explicitely not intended for module-use only, but for all other libraries.

## Key-Value split {#sec:libutil:kvsplit}

This helper implements functionality to split key-value string into two parts.
It was introduced to simplify commandline specification for header fields (see
@lst:kvsplit:headerspec).

```{#lst:kvsplit:headerspec .bash .numberLines caption="Headerfield spec"}
imag store create --path /some.entry entry --header field=foo
#                                                   ^^^^^^^^^
```

It is implemented by introducing a `KeyValue` type which is generic over Key
and Value. This type gets implemented `KeyValue<String, String> for String` to
be able to split a `String` into two `String` objects, key and value
respectively. The implementation is realized via Regex.

The `KeyValue` type implementes `Into<(K, V)>` for convenience.



