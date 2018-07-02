## libimagcontacts

The contact library basically only creates references to the actual
vcard files, though it also can parse (via the `vobject` crate) the
information and return it from an entry directly.

The architecture of indirections is as follows:

```{.numberLines}

+--------------------------------+
|                                |
|     Store, as ContactStore     |
|                                |
+----------------+---------------+
                 |
                 | Provides access to
                 |
+----------------v---------------+
|                                |
|   (FileLock)Entry as Contact   |
|                                |
|      which is actually a:      |
|                                |
|     (FileLock)Entry as Ref     |
|                                |
+----------------+---------------+
                 |
                 | refers to
                 |
+----------------v---------------+
|                                |
|   vcard file (outside store)   |
|                                |
+----------------+---------------+
                 |
                 | contains
                 |
+----------------v---------------+
|                                |
|            vcard data          |
|                                |
+--------------------------------+

```

