Language documentation for the imag-ids query language
======================================================

The query language imag-ids supports is rather simple.
It can be used to filter the printed imag ids by the values in the header of the
entries. It has no way to access the content of the entries (yet).

Following is a BNF-like structure shown how the language definition works.
This definition may change over time, as the language grews more powerful.

```ignore
query = filter (operator filter)*

filter = unary? ( (function "(" selector ")" ) | selector ) op val

unary = "not"

op =
    "is"  |
    "in"  |
    "=="  |
    "eq"  |
    "!="  |
    "neq" |
    ">="  |
    "<="  |
    "<"   |
    ">"   |
    "any" |
    "all"

val = val | listofval

val         = string | int | bool
listofval   = "[" (val ",")* "]"

operator =
    "or"      |
    "or_not"  |
    "and"     |
    "and_not" |
    "xor"

function =
    "length" |
    "keys"   |
    "values"
```

A "string" quoted with double-quotes.
A "val" does not yet support floats.
