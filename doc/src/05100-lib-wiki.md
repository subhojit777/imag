## libimagwiki

The wiki library implements a complete wiki for personal use.

This basically is a note-taking functionality combined with linking.

### Layout

The basic structure and layout is as simple as it gets:

`/wiki` holds all wikis. The default wiki is `/wiki/default`. Below that there
are entries. Entries can be in sub-collections, so
`/wiki/default/cars/mustang` could be an entry.


``` {.numberLines}

+-------------+
|             |
|  WikiStore  |
|             |
+------+------+
    1  |
       |
       | n
+------v------+
|             |
|    Wiki     |
|             |
+------+------+
     1 |
       |
       | n
+------v------+
|             | n
|    Entry    <------+
|             |      |
+------+------+      |
     1 |             |
       |             |
       |             |
       +-------------+
```

The store offers an interface to get a Wiki. The wiki offers an interface to get
entries from it.

Each Entry might link to a number of other entries _within the same wiki_.
Cross-linking from one wiki entry to an entry of another wiki is technically
possible, but not supported by the Entry itself (also read below).

When creating a new wiki, the main page is automatically created.

### Autolinking

The `Entry` structure offers an interface which can be used to automatically
detect links in the markdown.
The links are then automatically linked (as in `libimagentrylink`).

