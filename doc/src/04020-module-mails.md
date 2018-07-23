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

    imag mail track <path> [opts...]  # track a new mail, mail file passed as path
    imag mail scan <path> [opts...]   # scan a maildir and track all untracked mails
    imag mail box <name|path>         # work with the mailbox specified by <name|path>, name mappings from config
    imag mail list <args...>          # list mails in a given mailbox for a given account or the default account
    imag mail show <args...>          # open new mails in the pager
    imag mail thread list <args...>   # list mails from a thread
    imag mail thread show <args...>   # open new mails from a thread in the pager or call a script with them
    imag mail new <args...>           # craft a new mail and safe it in the <outgoing> folder
    imag mail send <args...>          # send emails from the outgoing folder, optionally also move them to archice boxes
    imag mail mv <srcbox> <dstbox>    # move a mail (or thread) from one mailbox to another

