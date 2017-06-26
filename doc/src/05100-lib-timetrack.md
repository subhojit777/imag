## libimagtimetrack

A library for tracking time events in the imag store.

### Store format

Events are stored with a store id like this:

```
/timetrack/<insert-date-year>/<insert-date-month>/<insert-date-day>/<insert-date-time>-<tag>.ext
```

Timetrackings contain

* a comment (optional, free text)
* a start date
* an end date
* a tag

by default and might be extended with more header fields as one likes.

The header of a timetrack "work" entry looks like this:

```toml
[event]
tag = "work"
start = "2017-01-02T03:04:05"
end = "2017-01-02T06:07:08"
```

Normal tags (as in `libimagentrytag`) are explicitely _not_ used for tagging,
so the user has the possibility to use normal tags on these entries as well.

The `tag` field is of type string, as for one tag, one entry is created. This
way, one can track overlapping tags, as in:

```bash
imag timetrack start foo
imag timetrack start bar
imag timetrack stop foo
imag timetrack start baz
imag timetrack stop bar
imag timetrack stop baz
```

The `end` field is, of course, only set if the event already ended.

### Library functionality

The library uses the `libimagentrydatetime::datepath::DatePathBuilder` for
building `StoreId` objects.

The library offers two central traits:

* `TimeTrackStore`, which extends a `Store` object with functionality to
  create `FileLockEntry` objects with a certain setting that is used for
  time-tracking, and
* `TimeTracking`, which extends `Entry` with helper functions to query the
  entry-metadata that is used for the time tracking functionality

The library does _not_ provide functionality to implement `imag-timetrack` or
so, as the core functionality is already given and the commandline application
can implement the missing bits in few lines of code.

Aggregating functionality might be provided at a later point in time.

