## libimaglog

A small extension over libimagdiary which strips down the functionality of
libimagdiary to some defaults for writing a `log` (a tumbleblog like diary)
with rather short messages.

Provides only basic functionality over libimagdiary, most notably the
"log.is_log" header entry, so the `imag-log` CLI can distinguish between
"logs" and "diary entries".

