## Calendar {#sec:modules:calendar}

The calendar module implements a commandline calendar like khal. The calendar
data itself is retrieved from icalendar files which should be located outside of
the imag store. imag does not handle syncing of these files. `vdirsyncer` may be
your tool of choise here.

imag can show events from the calendar(s) like any other commandline calendar
tool and of course can also add, delete or edit entries  (interactively or via
commandline parameters).

### Internals

What imag does internally is described in this section.

imag creates one entry in the store for one `icalendar` file. These entries are
basically references to the real data. If an  `icalendar` file is removed from
the filesystem, imag does not delete it from the sfore if not told explicitely.

