## libimagstore

The store is the heart of everything. Here lives the data, the complexity and
the performance bottleneck.

The store offeres read/write access to all entries, a hook system to do
on-the-fly modification of incoming/outgoing files and so on.

The store itself does not offer functionality, but has a commandline interface
"imag-store" which can do basic things with the store.

