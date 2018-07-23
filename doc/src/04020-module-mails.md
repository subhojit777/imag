## Mails {#sec:modules:mails}

The Mails module implements a commandline email client. Emails can be written
(via `$EDITOR`) and viewed, also in threads. Emails can be crawled for creating
new contacts.

A Text User Interface is not planned, but might be there at some point.

The mail module implements a minimal Email client. It does not handle IMAP
syncing or SMTP things, it is just a _viewer_ for emails (a MUA).

The goal of the initial implementation is only a CLI, not a TUI like mutt
offers, for example (but that might be implemented later). As this is an imag
module, it also creates references to mails inside the imag store which can be
used by other tools then (for example `imag-link` to link an entry with a mail -
or the imag entry representing that mail).

So this module offers functionality to read (Maildir) mailboxes, search for and
list mails and mail-threads and reply to mails (by spawning the `$EDITOR`).

Outgoing mails are pushed to a special directory and can later on be send via
`imag-mail` which calls a MTA (for example msmtp) and also creates store entries
for the outgoing mails.

### CLI

The CLI of the imag-mail module is planned as follows:

* imag mail

  Requires configuration:
    * mail.outgoingbox
    * mail.accounts.<account>.maildirroot

    -A, --account   - Specify the "account" to use for the opperation.
                      A account is nothing more than a mapping of a name to a
                      path where the Maildirs are located.  It is specified in
                      the imag configuration file.
                      For example:

                        private -> ~/.mails/personal

                      If none is specified, the configuration is searched for a
                      "maildir" setting (where all Maildirs are found in). If
                      there is no such setting, imag-mail fails.
                      (or: read in documentation for --box).

    -B, --box       - Specify a temporary location for a Maildir to work with.

* imag mail track <path> [opts...]
  Track a new mail, mail file passed as path

    --refind        - re-find messages. Loads all messages which are known to imag
                      and compares identifiers, to update the imag-internal cache if
                      a mail got moved

    -O, --output-id - print the store id for the new (or in case of --refind
                      updated) imag entry

* imag mail scan <path> [opts...]
  Scan a maildir and track all untracked mails

    --refind        - re-find messages. Loads all messages which are known to imag
                      and compares identifiers, to update the imag-internal cache if
                      a mail got moved

    -O, --output-id - print the store id for the new (or in case of --refind
                      updated) imag entry

* imag mail list <args...>
  List mails in a given mailbox for a given account or the default account

    -S, --style     - print messages in a certain style
                      Available:
                        - 'linewise'
                        - 'thread'

    -g, --grep      - Filter by grepping for a pattern in body and subject

    -d, --daterange - Filter by date(range)

    -F, --filter    - Filter by passed filter

        --thread    - Print only messages from the same thread as the found ones

    --format=<fmt>  - Format mails for showing.
                      --format always colorizes output (specify color in config)
                      except when using --no-pager or piping output.

                      When --tree is passed, the format is applied to the
                      fragment _after_ the tree graphic.

                      Default mode is 'default'.

                      Modes:
                        - 'subject': <Subject>
                        - 'simple': <From>: <Subject>
                        - 'default': <Date> - <From>: <Subject>
                        - 'fmt:<fmt>' format with passed format

                      Additional formats can be specified via the configuration
                      file. If a format has the same name as a predefined one,
                      the config overrides the predefined formats.

    --color         - Colorize output (default).
    --no-color      - Do never colorize output.

* imag mail show <args...>
  Show mail(s) - either in pager or by printing them to stdout.

    Mails are specified by message id or imag entry

    --refind        - If a imag entry is passed but the mail file is not there,
                      try to re-find it.

    --refind-in     - Same as --refind, but a path to a Maildir or a tree of
                      Maildirs might be passed to narrow down search space.

    -C, --concat    - Open all mails in one pager (by concatenating them)
                      instead of one pager per message.

    --pager         - Use pager to show mails (default).

    --no-pager      - Do not use pager to show mails.

    --multipager    - Pass all mails as arguments to one pager call instead of
                      calling the pager on each mail individually (default).
                      Only possible with --pager.

    --no-multipager - Disable --multipager.
                      Only possible with --pager.

    --format=<fmt>  - Format mails for showing.
                      --format always colorizes emails (specify color in config)
                      except when using --no-pager or piping output.

                      Modes:
                        - 'simple': Remove headers, except
                            From, To, Cc, Subject, Date,
                            Message-Id/References/In-Reply-To
                        - 'simple-imag': Same as 'simple' but also show imag
                          entry id.
                        - 'print': Show everything
                        - 'full': Show everything and add imag entry id
                        - 'minimal': Remove headers, except From, To, Cc, Subject, Date,
                        - 'tiny': Remove headers, except From, To, Subject
                        - 'fmt:<fmt>' format with passed format

                      Additional formats can be specified via the configuration
                      file. If a format has the same name as a predefined one,
                      the config overrides the predefined formats.

    --no-format     - Disable all formatting (same as --pretty=print and
                      disabling color output).

    --color         - Colorize output (default).
    --no-color      - Do never colorize output.

* imag mail new <args...>
  Craft a new mail and safe it in the <outgoing> folder

  Requires configuration:
    * mail.accounts.<account>.draftbox
    * mail.accounts.<account>.outgoingbox

        --outbox    - Specify the outbox for where the new mail should be stored
                      in, if it is not given in the config (or to override it)

        --to        - Specify to whom to send.
                      If the specified string does not contain a valid email
                      address, `imag contact find` is used to find the email
                      address (if not suppressed via --no-autofind).
                      Multiple allowed.

        --cc        - Specify to whom to send in CC.
                      If the specified string does not contain a valid email
                      address, `imag contact find` is used to find the email
                      address (if not suppressed via --no-autofind).
                      Multiple allowed.

        --bcc       - Specify to whom to send in BCC.
                      If the specified string does not contain a valid email
                      address, `imag contact find` is used to find the email
                      address (if not suppressed via --no-autofind).
                      Multiple allowed.

    --no-autofind   - Do not automatically find contacts
                      with `imag contact find`.

        --fcc       - Specify to store a copy of the mail somewhere.
                      Multiple allowed.

        --subject   - Specify subject.

        --gpg-sign  - Sign with gpg.

        --gpg-crypt - Crypt with gpg to all recipients.

        --no-track  - Do not track new mailfile with imag.

    -D, --draft     - Do not safe in "outgoing" box but rather in "draft" box.

* imag mail compose <args...>
  Same as 'new'.

* imag mail fetch <args...>
  Fetch emails

  Requires configuration:
    * mail.fetchcommand or mail.accounts.<account>.fetchcommand
    * mail.postfetchcommand or mail.accounts.<account>.postfetchcommand (optional)

    --all           - Fetch for all accounts
    --boxes         - Fetch only some boxes (does not work with --all)

* imag mail send <args...>
  Send emails from the outgoing folder, also move them to 'sent' boxes

  Requires configuration:
    * mail.accounts.<account>.outgoingbox
    * mail.accounts.<account>.sentbox
    * mail.sendcommand or mail.accounts.<account>.sendcommand
    * mail.postsendcommand or mail.accounts.<account>.postsendcommand (optional)

    --outbox        - Specify the outbox for where the mails that are about to
                      be send are stored in, if it is not given in the config
                      (or to override it).

    --sentbox       - Specify the sentbox for where the sent mails should be
                      moved after sending them, if it is not given in the config
                      (or to override it).

    --no-move-sent  - Do not move mail to the "sent" folder after sending it.

    --confirm       - Confirm each mail before sending (default).

    --no-confirm    - Do not confirm each mail before sending.

    --no-track      - Do not track mailfile with imag. Does only work if `imag
                      mail new` was invoked with `--no-track` (so that the mail
                      is not tracked already).

* imag mail mv <src mail> <dstbox>
  Move a mail to another mailbox

    --thread        - Move the complete thread of emails belonging to the
                      specified mail.

    --no-track      - Do not track new mailfile with imag. Does not work if
                      mailfile is already tracked with imag.

* imag mail find <args...>
  Search for a mail (by header field (msgid, from, to, cc, subject, date,
  date-range), body, ...)

    --msgid
    --no-msgid
    --from
    --no-from
    --to
    --no-to
    --cc
    --no-cc
    --subject
    --no-subject
    --date
    --no-date
    --body
    --no-body
    --daterange     - Toggle where to look at

    --print-entryid     - Print imag entry id when finding mail
    --no-print-entryid  - Do not print imag entry id when finding mail (default).

    --print=<what>  - What to print for the found mails.
                      Valid values:
                        - msgid
                        - subject
                        - from
                        - cc
                        - to
                        - date
                        - filepath (default)

* imag mail reply <args...>
  Reply to an email.

  Requires configuration: mail.accounts.<account>.outgoingbox

  Specify the mail to reply to by msgid, filepath or imag entry id.

    --add-to
    --add-cc
    --add-bcc       - Add another recipient. Multiple allowed.

    --no-track      - Do not track new mailfile with imag.

