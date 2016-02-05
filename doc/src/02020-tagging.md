## Tagging entries {#sec:thestore:tagging}

A store entry MAY be tagged. A tag is a String which matches the
regular expression in @lst:tagging:regex

```{#lst:tagging:regex .numberLines caption="Regular Expression for Tags"}
/^[a-zA-Z]([a-zA-Z0-9_-]*)$/
```

Tags MUST BE stored in the header section "imag" in the key "tags" as an Array
of Strings.
The tags MUST BE sorted in alphabetical order.

