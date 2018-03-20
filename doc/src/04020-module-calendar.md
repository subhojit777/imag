## Calendar {#sec:modules:calendar}

The calendar module implements a commandline calendar like khal. The calendar
data itself is retrieved from icalendar files which should be located outside of
the imag store. imag does not handle syncing of these files. `vdirsyncer` may be
your tool of choise here.

imag can show events from the calendar(s) like any other commandline calendar
tool and of course can also add, delete or edit entries  (interactively or via
commandline parameters).

### Usage

imag creates three different kind of entries in the store:

1. A "collection" for each directory containing .ical filesystem
1. A "calendar" for each .ical file
1. A "event" for each event in an .ical file

A collection can be added to the imag store via

```
imag calendar collection add /path/to/icaldir
```

This automatically adds a `collection` which refers to the directory.
It also reads all files in that directory
adding `calendar` objects to the imag store for each of them.
After that, each `calendar` is read for events and for each event, imag
creates an `event` object in the imag store.

Each `collection` links to a number of `calendar` objects and each `calendar`
object links to a number of `event` objects inside imag.

```
imag calendar collections
# or
imag calendar collection --list
```

shows a list of existing collections.

