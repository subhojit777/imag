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

imag creates three different kind of entries in the store:

1. A "collection" for each directory containing .ical filesystem
1. A "calendar" for each .ical file
1. A "event" for each event in an .ical file


