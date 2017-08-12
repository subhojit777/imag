## Mails {#sec:modules:mails}

The Mails module implements a commandline email client. Emails can be written (via `$EDITOR`) and viewed, also in threads. Emails can be crawled for creating new contacts.

A Text User Interface is not planned, but might be there at some point.

The mail module implements a minimal Email client. It does not handle IMAP syncing or SMTP things, it is just a _viewer_ for emails (a MUA).

The goal of the initial implementation is only a CLI, not a TUI like mutt offers, for example (but that might be implemented later). As this is an imag module, it also creates references to mails inside the imag store which can be used by other tools then (for example `imag-link` to link an entry with a mail - or the imag entry representing that mail).

So this module offers functionality to read (Maildir) mailboxes, search for and list mails and mail-threads and reply to mails (by spawning the `$EDITOR`).

Outgoing mails are pushed to a special directory and can later on be send via `imag-mail` which calls a MTA (for example msmtp) and also creates store entries for the outgoing mails.

