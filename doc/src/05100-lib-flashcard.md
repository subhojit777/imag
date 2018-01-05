## libimagflashcard

Flash card program.

Asks the user a question and uses heuristics to check whether the answer is
correct (string matching).

Each flashcard is one imag entry. Entries can be grouped together (imag
links). A flashcard can be in several groups.
The Sessions-Feature can be used to track progress and add a value to each
flashcard, so when starting a new learning session, better learned cards show
up less often.

### Architecture

The architecture is layed out as follows:

```
+-------+       +-------+       +------+
| Store +------>+ Group +------>+ Card |
+-------+       +---+---+       +------+
                    |
                    |
                    v
               +----+----+
               | Session |
               +---------+
```

Via the `Store`, a `Group` can be fetched/created.
Via a `Group`, `Card`s can be created.
A `Group` can also be used to create a `Session`, which records the current
learning state (which card was answered correctly, which was answered wrong
during a learning-session).

A `Group` and a `Card` are linked (via `libimagentrylink`).
A `Session` is linked to both its `Group` (via `libimagentrylink`)
and all `Cards` accessed during the session.
The link from a `Session` to the `Card` objects is not done via
`libimagentrylink` because the `Session` object must store a list of correctly
and wrong answered questions, so it holds the names of the `Card`s in a
non-`libimagentrylink` way.

### Names

A `Group` has a StoreId at `flashcard/<groupname>/group`.
All `Card`s of the group are stored in

```
flashcard/<group name>/cards/<hash of the question>
```

Thus, each question is unique in its group.
`Session` objects are located in

```
flashcard/sessions/<group name>/<datetime>
```

