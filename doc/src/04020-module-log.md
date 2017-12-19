## Log {#sec:modules:log}

The "imag-log" module is a lightweight interface to the "imag-diary" command.

It is intended as a tumbeblog-like diary, where one does not care to fire up
an editor and type in a long text, but rather type a few words and forget
about it:

### Usage

Logs can be created via an entry in the configuration file in the section `log`:

```
[log]
logs = ["work", "hobby", "music"]
default = "hobby"
```

The `default` key is required and the name which is used here _must_ appear in
the `logs` array.

In the above configuration snippet, the logs `work`, `hobby` and `music` are
created. The user may now log to one of these logs with:

```
imag log --to <logname> "Some message"
# or
imag log -t <logname> "Some message"
# or, to the default log:
imag log "Some message"
```

Logs can be read by naming the log:

```
imag log show work
```

which prints one log per line (including time it was logged).

