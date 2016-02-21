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

## Error tracing {#sec:libutil:errortrace}

The error tracing functions are functions which help printing an error chain
to the user.

It allows to trace nested errors like @lst:errtrace:exampleerror to the user
in a backtrace-ish way (@lst:errtrace:exampletrace).

```{#lst:errtrace:exampleerror.rust .numberLines caption="Error chain"}
ErrA::new(a_errorkind,
          Some(Box::new(ErrB::new(b_errorkind,
                                  Some(Box::new(ErrC::new(c_errorkind,
                                                          None)))
                                  )))
          )
```

The variants of the function allow limiting the trace to a certain depth or
printing the error trace to the debug output stream.

```{#lst:errtrace:exampletrace .numberLines caption="Error trace"}
[Error][c_errorkind]: Some C-error text -- caused:
[Error][b_errorkind]: Some B-error text -- caused:
[Error][a_errorkind]: Some A-error text
```

## Variant generator {#sec:libutil:vargen}

The `generate_variants()` function can be used to generate variants of a base
vector value.

```{#lst:vargen:exampleuse .rust .numberLines caption="Variant generation"}
let base = 1;
let vars = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
let res = generate_variants(base, vars, &|base, var| base + var);

assert!(res == vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11])
```

As shown in @lst:vargen:exampleuse (from the tests), can this function
be used to generate values from a base value.

