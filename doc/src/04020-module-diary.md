## Diary {#sec:modules:diary}

The Diary module.

### Description

The diary module is for keeping your diary notes. It offers a self-implemented
diary which creates the entries in the store.

As of now there is only the possibility to create daily entries, but the
possibility to implement hourly or even minutely entries is there.

The module offers commands to create, delete, edit and list diary entries.

### Backends

At this moment, only the imag store is an available backend and therefor diary
entries are written to the imag store.

There is no implementation for other diary software planned _yet_, but there
might be a [jrnl](http://jrnl.sh/) backend some time, but not as long as `jrnl`
does not provide a multi-file storage system.

