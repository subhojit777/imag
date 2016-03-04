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

| Type          | Meaning                                          |
| :------------ | :----------------------------------------------- |
| Entry         | Entity on the Filesystem, File                   |
| EntryContent  | User-Content of the Entry                        |
| EntryHeader   | Header of the Entry                              |
| Store         | Store interface                                  |
| FileLockEntry | Handle to an Entry                               |

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

The entry type MUST export functions to set

- The header object
- The content object

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

## Hook system {#sec:libstore:hooks}

The store library includes a hook system, which can be used to execute arbitrary
code before or after the store was accessed. The following hooks are available:

* `PreReadHook`
* `PostReadHook`
* `PreCreateHook`
* `PostCreateHook`
* `PreUpdateHook`
* `PostUpdateHook`
* `PreDeleteHook`
* `PostDeleteHook`

These are called "Hook positions" in the following.

Which are executed before or after the store action is executed. The `Pre`-Hooks
can deny the execution by returning an error. The `Post`-Hooks can (for the
appropriate store actions) alter the hook result.

Registering hooks with the store is implemented via functions on the `Store`
type itself. Hooks MUST NEVER be removed from the `Store` object during runtime,
only adding hooks to the store is allowed.

As the hooks are simply trait objects, one is able to implement arbitrary hooks,
for example

* Simple consistency-checks for the store
* Version control system adaption for the store (via git for example)
* Encryption of store entries (via gnupg for example)
* Automatic backup on every change to the store (via rsnapshot for example)

Some hooks MAY be shipped with the imag source distribution and be enabled by
default.

Execution order of the hooks is a not-yet-solved problem.

### Hook-Aspects {#sec:libstore:hooks:aspects}

Each hook can be assigned to an "Aspect". There MAY BE zero or more aspects for
each Hook position. Aspects can be sorted and configured via the configuration
file, whereas each aspect has its own configuration section:

```{#lst:hooks:aspects:cfg .toml .numberLines caption="Hook config section"}
[hooks]

[[aspects]]

// Defines order of aspects for the pre-read hook position
pre-read-aspects  = [ "misc" ]

// Defines order of aspects for the post-read hook position
post-read-aspects = [ "decryption" ]

// ...

// configuration for the "misc" hook aspect
[[misc]]
parallel-execution = true

// configuration for the "decryption" hook aspect
[[decryption]]
parallel-execution = false
```

Aspects are executed in the same order they appear in the configuration. Aspects
_could_ be sorted in different order for each hook position.

Aspects where parallel execution is enabled MAY BE executed in sequence if one
of the hooks wants mutable access to the data they hook into.

Hooks can then be assigned to one hook aspect. Hooks MUST never be assigned to
more than one hook aspect. Hooks which are not assigned to any aspect MUST never
be executed.

```{#lst:hooks:cfg .toml .numberLines caption="Hook configuration"}
[hooks]

// decrypt hook with gnupg. An appropriate "gnupg-encrypt" hook must be defined
// to be fully operational, of course
[[gnupg-decrypt]]
aspect = "decryption"
key = "0x123456789"

// version control hook. Sorted into aspect "misc" here.
[[git]]
aspect = "misc"

// ...
```

Hooks MAY HAVE arbitrary configuration keys.

