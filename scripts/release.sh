#!/usr/bin/env bash

CRATES=(
    ./lib/etc/libimagutil
    ./lib/etc/libimagtimeui
    ./lib/core/libimagerror
    ./lib/core/libimagstore
    ./lib/etc/libimagnotification
    ./lib/etc/libimaginteraction
    ./lib/core/libimagrt
    ./lib/entry/libimagentryfilter
    ./lib/entry/libimagentrycategory
    ./lib/entry/libimagentryannotation
    ./lib/entry/libimagentrylink
    ./lib/entry/libimagentrytag
    ./lib/entry/libimagentrygps
    ./lib/entry/libimagentrylist
    ./lib/entry/libimagentryedit
    ./lib/entry/libimagentryref
    ./lib/entry/libimagentryview
    ./lib/entry/libimagentrymarkdown
    ./lib/entry/libimagentrydatetime
    ./lib/domain/libimagbookmark
    ./lib/domain/libimaghabit
    ./lib/domain/libimagnotes
    ./lib/domain/libimagcontact
    ./lib/domain/libimagdiary
    ./lib/domain/libimagtimetrack
    ./lib/domain/libimagtodo
    ./lib/domain/libimagmail
    ./bin/domain/imag-habit
    ./bin/domain/imag-diary
    ./bin/domain/imag-contact
    ./bin/domain/imag-notes
    ./bin/domain/imag-bookmark
    ./bin/domain/imag-timetrack
    ./bin/domain/imag-mail
    ./bin/domain/imag-todo
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
    ./bin/core/imag
)

for crate in ${CRATES[*]}; do
    echo -e "\t[CARGO][CHECK  ]\t$crate"
    RUST_BACKTRACE=1 cargo publish --manifest-path $crate/Cargo.toml || exit 1
    echo -e "\t[Waiting...]"
    sleep 15
done

