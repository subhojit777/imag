# libstore {#sec:libstore}

<!--
    Store functionality
-->

The "libstore" MUST define the programming-language level interface to the
store on the file system.
The library therefor MUST define and export types which can be used to get data
from the filesystem.

## Types {#sec:libstore:types}

The types in @tbl:libstore:types are exported by the library.

| Type         | Meaning                                          |
| :----------- | :----------------------------------------------- |
| Entry        | Entity on the Filesystem, File                   |
| EntryContent | User-Content of the Entry                        |
| EntryHeader  | Header of the Entry                              |
| Store        | Store interface                                  |

Table: Types the store library exports {#tbl:libstore:types}

Each of these types MUST export functions to work with the data the objects of
the types contain.

### Entry {#sec:libstore:types:entry}

The `Entry` type MUST hold the following content:

- A path where on the filesystem the acutal file lives
- An instance of `EntryContent` as interface to the content of the file
  (@sec:libstore:types:entrycontent).
- An instance of `EntryHeader` as interface to the header of the file
  (@sec:libstore:types:entryheader).

The entry type MUST export functions to get

- The content object
- The header object
- The path of the actual file

### EntryContent {#sec:libstore:types:entrycontent}

The `EntryContent` type is an type-alias for `String`.

### EntryHeader {#sec:libstore:types:entryheader}

The `EntryHeader` type is an wrapper around the type, the TOML-Parser library
exports.

It SHOULD contain utility functions to work with the header in a convenient way.

### Store {#sec:libstore:types:store}

The `Store` type MUST represent the interface to the store on the filesystem.
It MUST contain CRUD-functionality to work with the entries in the store.
It MUST contain a variable which contains the path of the store on the
filesystem which is represented by an object of this type.
It also MUST contain a getter for this variable.
It MUST NOT contain a setter for this variable, as changing the store while the
programm is running is not allowed.

The `Store` object MAY contain caching functionality.
If the `Store` type contains caching functionality for filesystem operations,
the interface MUST NOT differ from the non-caching interface.
If the `Store` type contains caching functionality, these functionality MUST NOT
BE visible to the user of the library.

