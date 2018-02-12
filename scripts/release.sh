#!/usr/bin/env bash


CRATES=(
    ./lib/etc/libimagutil
    ./lib/etc/libimagtimeui
    ./lib/core/libimagerror
    ./lib/core/libimagstore
    ./lib/etc/libimagnotification
    ./lib/etc/libimaginteraction
    ./lib/core/libimagrt
    ./lib/entry/libimagentrycategory
    ./lib/entry/libimagentrylink
    ./lib/entry/libimagentrytag
    ./lib/entry/libimagentryfilter
    ./lib/entry/libimagentrygps
    ./lib/entry/libimagentryedit
    ./lib/entry/libimagentryview
    ./lib/entry/libimagentrydatetime
    ./lib/entry/libimagentryutil
    ./lib/entry/libimagentryref
    ./lib/entry/libimagentrymarkdown
    ./lib/entry/libimagentryannotation
    ./lib/domain/libimagbookmark
    ./lib/domain/libimagcalendar
    ./lib/domain/libimaghabit
    ./lib/domain/libimagnotes
    ./lib/domain/libimagcontact
    ./lib/domain/libimagdiary
    ./lib/domain/libimaglog
    ./lib/domain/libimagtimetrack
    ./lib/domain/libimagtodo
    ./lib/domain/libimagmail
    ./lib/domain/libimagwiki
    ./bin/domain/imag-habit
    ./bin/domain/imag-diary
    ./bin/domain/imag-contact
    ./bin/domain/imag-notes
    ./bin/domain/imag-bookmark
    ./bin/domain/imag-timetrack
    ./bin/domain/imag-mail
    ./bin/domain/imag-todo
    ./bin/domain/imag-log
    ./bin/domain/imag-wiki
    ./bin/core/imag-ref
    ./bin/core/imag-gps
    ./bin/core/imag-diagnostics
    ./bin/core/imag-mv
    ./bin/core/imag-store
    ./bin/core/imag-tag
    ./bin/core/imag-grep
    ./bin/core/imag-annotate
    ./bin/core/imag-link
    ./bin/core/imag-view
    ./bin/core/imag-init
    ./bin/core/imag-edit
    ./bin/core/imag-ids
    ./bin/core/imag
)

for crate in ${CRATES[*]}; do
    echo -e "\t[CARGO][CHECK  ]\t$crate"
    RUST_BACKTRACE=1 cargo publish --manifest-path $crate/Cargo.toml || exit 1
    echo -e "\t[Waiting...]"
    sleep 15
done

