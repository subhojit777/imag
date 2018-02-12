## libimagcalendar

The calendar library basically only creates references to the actual
icalendar files, though it also can parse (via the `vobject` crate) the
information and return it from an entry directly.

The architecture of indirections is as follows:

```{.numberLines}

+--------------------------------+
|                                |
|    Store, as CalendarStore     |
|                                |
+----------------+---------------+
                1|
                 |
                 |*
+----------------v---------------+
|                                |
|       (FileLock)Entry as       |
|       CalendarCollection       |
|                                |
|      which is actually a:      |
|                                |
|     (FileLock)Entry as Ref     |
|                                |
+----------------+---------------+
                1|
                 |
                 |*
+----------------v---------------+
|                                |    Ref   +---------------------------+
|        (FileLock)Entry as      |          |                           |
|           CalendarFile         +----------> ical file (outside store) |
|                                |          |                           |
|        which is actually a     |          +-------------+-------------+
|                                |                        |
|      (FileLock)Entry as Ref    |                        | contains
|                                |                        |
+----------------+---------------+          +-------------v-------------+
                                            |                           |
                                            |          ical data        |
                                            |                           |
                                            +---------------------------+
```

