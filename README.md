# imag

Imag is a CLI PIM suite with a nice API-ish commandline interface, so you can
integrate it in your tools of coice (Editor, MUA, RSS reader, etc etc).

## How data is stored

The backends define how the data is stored and accessed. Each backend has
either Read-Write permission or Read-Only.

By now, only files are planned as storage backend.

File formats are

* JSON
* Markdown with YAML-Header

## Tools

Each tool has a storage "pool", which can be git version controlled. This can
then be used to sync devices. For example bookmarks should be git version
control, while mails should obviously not (they should also only be accessed
RO).

Each of the following modules has a short description including a table what
core features are required to get it working.

Note that all SHA512 hashes which appear in the following chapters are
constants and they should never change. Changing them would break things, as
the SHA512 hashes are used to be able to link data together.

### Bookmarks

Bookmarks should be stored in a simple format:

```json
{ "URL": "https://github.com", "tags": ["ducks", "r", "great"]}
```

Each file is one bookmark and the filename is a SHA512.

| Required core feature                 | Purpose       |
| :------------------------------------ | :------------ |
| Filesystem access (RW)                | Data storage  |
| JSON File backend                     | Data format   |
| Git backend                           | Data sync     |

### Contacts

Contacts are just read and indexed by `imag`, to create an internal index of
them and to be able to refer to.

| Required core feature                 | Purpose       |
| :------------------------------------ | :------------ |
| Filesystem access (RW)                | Data access   |
| vcard file format parsing             | Data access   |

### Calendar

Calendar are just read and indexed by `imag`, to create an internal index of
them and to be able to refer to.

| Required core feature                 | Purpose       |
| :------------------------------------ | :------------ |
| Filesystem access (RW)                | Data access   |
| ical file format parsing              | Data access   |

### Mail

`imag` should be able to index all your mail and make them accessible through
its interfaces, so you can link contacts, calendar entries, wiki pages, todo
notes, etc etc. to your mail.

Some of these things (like linking contacts, calendar entries) should be
linked automatically.

| Required core feature                 | Purpose       |
| :------------------------------------ | :------------ |
| Filesystem access (RO)                | Data access   |
| Maildir file format parsing           | Data access   |
| mbox file format parsing (later)      | Data access   |
| Internal storage database             | Data indexing |
| JSON File backend                     | Database      |
| Editor calling                        | Editing       |
| Mail-Client calling                   | Editing       |

### Personal Wiki

`imag` should contain a complete personal wiki software. It should contain of
simple markdown files which have a YAML header, nothing too special for the
first step.

| Required core feature                 | Purpose       |
| :------------------------------------ | :------------ |
| Filesystem access (RW)                | Data access   |
| YAML parsing                          | Data parsing  |
| Markdown parsing                      | Data parsing  |
| Git backend                           | Data sync     |
| Editor calling                        | Editing       |

Some more ideas:

- Extract URLs and put them into store as Bookmarks

### Todo-List

`imag` should also contain a full todo-tool. I'm thinking about integrating
taskwarrior through a wrapper here.

| Required core feature                 | Purpose       |
| :------------------------------------ | :------------ |
| Filesystem access (RO)                | Data access   |
| Taskwarrior backend                   | Data parsing  |
| Git backend                           | Data sync     |
| Editor calling                        | Editing       |

### Shoppinglist

Simply dot-and-tick lists.

| Required core feature                 | Purpose       |
| :------------------------------------ | :------------ |
| Filesystem access (RW)                | Data access   |
| YAML parsing                          | Data parsing  |
| Markdown parsing                      | Data parsing  |
| Git backend                           | Data sync     |
| Editor calling                        | Editing       |

### Bibliography management

BibTex would be the first step, maybe we should be able to add the actual PDFs
here as well... didn't waste too much thoughts on this by now. If we have the
PDF data, we need git-annex.

| Required core feature                 | Purpose       |
| :------------------------------------ | :------------ |
| Filesystem access (RW)                | Data access   |
| BibTex parsing                        | Data parsing  |
| Git backend                           | Data sync     |
| Git-annex backend                     | Data sync     |
| Editor calling                        | Editing       |

### News (RSS)

Just indexing, reading news is not task of `imag` and so isn't syncing.

| Required core feature                 | Purpose       |
| :------------------------------------ | :------------ |
| Filesystem access (RO)                | Data access   |
| RSS parsing                           | Data parsing  |
| Git backend                           | Data sync     |

# License

This code is released under terms of GNU GPLv2.

