## Counter {#sec:modules:counter}

The Counter module helps you counting things.

### Description

In its current state the counter module is capable of simple counting. You can
create, list and delete counters which are simply numbers and incremet,
decrement, set and reset them.

Future plans include counting functionality which is able to save date and
possibly timestamp of your increments/decrements, so you can export this and use
(for example) R to visualize this data.

Filters for selecting only certain time ranges when listing/exporting your
counters will be added as well.

### Examples

Here are some examples how to use the counter module:

```bash

imag counter create --name example --initval 42 # or: -n example -i 42
imag counter --inc example # or -i example
imag counter --reset example
imag counter --dec example # or -d example
```

### Backends

<!-- Backends the module supports including links to external resources -->

