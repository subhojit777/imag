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
your ical files and puts only a link and some hashsums (for refinding on content
move) to the store. Changes are not tracked via this model.  
If a (for example ical-)file gets removed, the store entry gets invalid and has
to be garbage-collected.

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

| Module       | Indexer | Header-Format | Content | Expl    | Dep         |
| :----------- | ------- | ------------- | ------- | ------- | ----------- |
| Bibliography | X       | JSON          | X       | .bib    | Notes       |
| Bookmarks    |         | JSON          |         |         |             |
| Calendar     | X       | JSON          |         |         |             |
| Contacts     | X       | JSON          |         |         |             |
| Image        | X       | JSON          |         |         | Notes       |
| Mail         | X       | JSON          |         |         |             |
| Movie        | X       | JSON          |         |         | Notes       |
| Music        | X       | JSON          |         |         | Notes       |
| News         | X       | JSON          |         |         |             |
| Notes        |         | YAML          | X       | Content |             |
| Shoppinglist |         | YAML          |         |         | Notes, Todo |
| Todo         |         | YAML          |         |         | Notes       |
| Wiki         |         | YAML          | X       | Content |             |

Explanation:

- An "Indexer" Module does only index some data and stores only some information
  on how to content
- "Header-Format": Which format is chosen for the header. Basically: YAML if the
  user might want to edit the header, otherwise JSON (pretty).
- "Content" means that the content part of a store entry is used
- "Expl": What the content part of a file is used for. "Content" means simply
  user content.
- "Dep" means that the module uses this other module as a dependency for
  extending its own functionality

### External Dependencies

| Library        | Optional | Module        |
| :------------  | :------: | :-------      |
| vcard          |          | Contacts      |
| icalendar      |          | Calendar      |
| XDG            | X        | Contacts      |
|                | X        | Calendar      |
|                | X        | Notes         |
|                | X        | Mail          |
|                | X        | Wiki          |
|                | X        | Todo          |
|                | X        | Shopping List |
|                | X        | BibMan        |
|                | X        | Music         |
|                | X        | Movie         |
|                | X        | Image         |
| Markdown       |          | Notes         |
|                |          | Wiki          |
| Maildir        |          | Mail          |
| BibTex parsing | X        | BibMan        |
| git-annex      | X        | BibMan        |
|                | X        | Music         |
|                | X        | Movie         |
|                | X        | Image         |
| Exif           | X        | Image         |
| id3            | X        | Music         |
| RSS/Atom       |          | News          |

(Optional means that these things are optional for the main functionality, but
should be implemented at some point to offer powerful functionality)

### Bookmarks

Bookmarks should be stored in a simple format:

```json
{ "URL": "https://github.com", "TAGS": ["ducks", "r", "great"]}
```

| Required util feature                 | Purpose          |
| :------------------------------------ | :------------    |
| XDG-open (Browser)                    | External program |

### Contacts

Contacts are just read and indexed by `imag`, to create an internal index of
them and to be able to refer to.

### Calendar

Calendar are just read and indexed by `imag`, to create an internal index of
them and to be able to refer to.

### Notes

Just plain text notes.

### Mail

`imag` should be able to index all your mail and make them accessible through
its interfaces, so you can link contacts, calendar entries, wiki pages, todo
notes, etc etc. to your mail.

Some of these things (like linking contacts, calendar entries) should be
linked automatically.

### Personal Wiki

`imag` should contain a complete personal wiki software. It should contain of
simple markdown files which have a YAML header, nothing too special for the
first step.

Some more ideas:

- Extract URLs and put them into store as Bookmarks

### Todo-List

`imag` should also contain a full todo-tool. I'm thinking about integrating
taskwarrior through a wrapper here.

### Shoppinglist

Simply dot-and-tick lists, uses Todo and Notes in combination.

### Bibliography management

BibTex would be the first step, maybe we should be able to add the actual PDFs
here as well... didn't waste too much thoughts on this by now. If we have the
PDF data, we need git-annex.

### News (RSS)

Just indexing, reading news is not task of `imag` and so isn't syncing.

### Image

Just indexing photos.

### Video

Just indexing movies.

### Music

Just indexing music.

# License

This code is released under terms of GNU GPLv2.

