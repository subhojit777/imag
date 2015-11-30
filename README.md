# imag

Imag is a CLI PIM suite with a nice API-ish commandline interface, so you can
integrate it in your tools of coice (Editor, MUA, RSS reader, etc etc).

## Modules

All the modules have access to a shared store (which lives in `~/.imag/store/`
by default). Files are named after a schema:

    ~/.imag/store/<module_name>-<hashtype>-<hash>.imag

Where module name is "bookmark" for example. The hashtype is "UUID" by default
and the "hash" is a UUID then. Other types are possible as well:

- SHA1
- SHA512

for example (though it might not be a good idea to use SHA hashes,
see [Linking](#Linking))

Some modules might not want to store content there, for example you don't want
to put your icalendar files in there. So the calendar module just crawls through
your ical files and puts the scanned meta-information into the store. Of course,
if the content of the ical file changes, the store entry does not. It still
points (via its JSON content for example) to the same file. So changes are not
tracked (We can argue here whether we want to copy the contents to the store for
ical and vcard files, but we cannot argue on for example music files).

If a (for example ical-)file gets removed, the store entry gets invalid and has
to be garbage-collected.

>   The current model is not fixed yet. I'm thinking about copying .ical and
>   .vcard, basically all text files, to the store.
>   This is not possible for media files like music or movies, though. Also this
>   is not feasible for documents like .pdf or similar.

Each of the following modules has a short description including a table what
core features are required to get it working.

### Linking

The UUID/SHA hashes in the file names can be used to connect two store entries.
For example, an entry from the wiki could refer to a contact by a UUID, because
the file for the contact has a UUID in its name. These UUIDs are constant and
should not change.

A link chain like this

    Wiki -> Shopping List -> Calendar Entry
            |                       |
            |                       v
            |                       Contact -> Calendar Entry -> Bookmark
            v
            Wiki Entry -> Bookmark

is totally possible. Scenario: You have a Wiki entry of a recipe you like to
cook. The recipe is linked to a shopping list, where you want to buy the
ingredients. Of course, you want to do this on a specific date (calendar entry).
And you want to do this with your Girlfriend, so she (as a contact) is linked to
the calendar entry. Her Birthday is linked to her Contact and for her Birthday,
you saw something on amazon, so you bookmarked it and linked it to the calendar
entry.
On the other hand, the Shopping list has links to some other Wiki entry, because
it contains Kiwi, and you have notes about Kiwi, how to cook them properly. You
saw this on some website, so you linked to the website from your wiki entry, of
course.

### Header

So each file in the store has a certain format:

    ---
    <header>
    ---
    <content>

The header contains some structured data (either JSON or YAML), the module uses
to store data closely related to the content.

For example, the bookmark-module will use the header only and store URL and tags
in some JSON format. The Wiki module might use the Header for storing some meta
information about the wiki article (tags, category, maybe even links to other
store entries) and the content is the wiki article content written down by the
user.

The content is just text entered from the user. In the best case, the module
should't touch the content at all.

Here is a short overview what are the modules like:

| Module            | Indexer | Header  | Hdr-Format | Content |
| :---------------- | ------- | ------- | ---------- | ------- |
| Bookmarks         |         | X       | JSON       |         |
| Contacts          | X       | X       | JSON       |         |
| Calendar          | X       | X       | JSON       |         |
| Notes             |         | X       | YAML       | X       |
| Mail              | X       | X       | JSON       |         |
| Wiki              |         | X       | YAML       | X       |
| Todo              |         | X       | YAML       |         |
| Shoppinglist      |         | X       | YAML       |         |
| Bibliography      | X       | X       | JSON       |         |
| News              | X       | X       | JSON       |         |
| Image             | X       | X       | JSON       |         |
| Movie             | X       | X       | JSON       |         |
| Music             | X       | X       | JSON       |         |

Explanation:

- An "Indexer" Module does only index some data and store the indexed meta
  information in the store
- "Header" means that the header part of a store entry is used
- "Hdr-Format" Which format is chosen for the header. Basically: YAML if the
  user might want to edit the header, otherwise JSON (pretty).
- "Content" means that the content part of a store entry is used

### Bookmarks

Bookmarks should be stored in a simple format:

```json
{ "URL": "https://github.com", "tags": ["ducks", "r", "great"]}
```

| Required core feature                 | Purpose          |
| :------------------------------------ | :------------    |
| XDG-open (Browser)                    | External program |

### Contacts

Contacts are just read and indexed by `imag`, to create an internal index of
them and to be able to refer to.

| Required core feature                 | Purpose          |
| :------------------------------------ | :------------    |
| vcard file format parsing             | Data access      |
| XDG-open (Mail program)               | External Program |

### Calendar

Calendar are just read and indexed by `imag`, to create an internal index of
them and to be able to refer to.

| Required core feature                 | Purpose       |
| :------------------------------------ | :------------ |
| ical file format parsing              | Data access   |

### Notes

Just plain text notes.

| Required core feature                 | Purpose          |
| :------------------------------------ | :------------    |
| Linking to other store entries        | Data Link        |
| XDG-open (Editor)                     | External Program |

### Mail

`imag` should be able to index all your mail and make them accessible through
its interfaces, so you can link contacts, calendar entries, wiki pages, todo
notes, etc etc. to your mail.

Some of these things (like linking contacts, calendar entries) should be
linked automatically.

| Required core feature                 | Purpose          |
| :------------------------------------ | :------------    |
| Parser: Maildir                       | Data access      |
| Parser: mbox                          | Data access      |
| XDG-open (Mail program)               | External Program |
| XDG-open (Editor)                     | External Program |

### Personal Wiki

`imag` should contain a complete personal wiki software. It should contain of
simple markdown files which have a YAML header, nothing too special for the
first step.

| Required core feature                 | Purpose          |
| :------------------------------------ | :------------    |
| Parser: Markdown                      | Data parsing     |
| XDG-open (Editor)                     | External Program |
| XDG-open (Mail program)               | External Program |
| XDG-open (Browser)                    | External Program |

Some more ideas:

- Extract URLs and put them into store as Bookmarks

### Todo-List

`imag` should also contain a full todo-tool. I'm thinking about integrating
taskwarrior through a wrapper here.

| Required core feature                 | Purpose          |
| :------------------------------------ | :------------    |
| Library: taskwarrior                  | Implementation   |
| XDG-open (Editor)                     | External Program |

### Shoppinglist

Simply dot-and-tick lists.

| Required core feature                 | Purpose          |
| :------------------------------------ | :------------    |
| Module: Todo-List                     | Implementation   |
| XDG-open (Editor)                     | External Program |

- Sub-form of the Todo-List module

### Bibliography management

BibTex would be the first step, maybe we should be able to add the actual PDFs
here as well... didn't waste too much thoughts on this by now. If we have the
PDF data, we need git-annex.

| Required core feature                 | Purpose          |
| :------------------------------------ | :------------    |
| BibTex parsing                        | Data parsing     |
| Backend: git-annex                    | Data parsing     |
| XDG-open (Editor)                     | External Program |
| XDG-open (PDF viewer)                 | External Program |
| XDG-open (Office suite)               | External Program |

### News (RSS)

Just indexing, reading news is not task of `imag` and so isn't syncing.

| Required core feature                 | Purpose       |
| :------------------------------------ | :------------ |
| Parser: RSS                           | Data parsing  |
| Parser: Atom                          | Data parsing  |

### Image

Just indexing photos.

| Required core feature                 | Purpose          |
| :------------------------------------ | :------------    |
| Backend: git-annex                    | Data parsing     |
| Image metadata reading                | Data parsing     |
| XDG-open (Image viewer)               | External Program |

### Video

Just indexing movies.

| Required core feature                 | Purpose          |
| :------------------------------------ | :------------    |
| Backend: git-annex                    | Data parsing     |
| Movie metadata reading                | Data parsing     |
| XDG-open (Movie viewer)               | External Program |

### Music

Just indexing music.

| Required core feature                 | Purpose          |
| :------------------------------------ | :------------    |
| Backend: git-annex                    | Data parsing     |
| Music metadata reading                | Data parsing     |
| XDG-open (Music player)               | External Program |

# License

This code is released under terms of GNU GPLv2.

