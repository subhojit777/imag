#!/usr/bin/env bash

source $(dirname ${BASH_SOURCE[0]})/../../tests/utils.sh
source $(dirname ${BASH_SOURCE[0]})/utils.sh

default_entry() {
    cat <<EOS
---
[imag]
links = []
version = "0.2.0"
---

EOS
}

entry_linked_to() {
    cat <<EOS
---
[imag]
links = [$1]
version = "0.2.0"
---

EOS
}

mktestentry() {
    mkdir -p ${STORE}
    default_entry > ${STORE}/$1
}

test_link_modificates() {
    mktestentry "test"
    mktestentry "test2"

    imag-link internal add --from "test" --to "test2"

    if [ "$(default_entry)" == "$(cat_entry 'test')" ] ||
       [ "$(default_entry)" == "$(cat_entry 'test2')" ]
    then
        err "Entry was unmodified after linking"
        return 1;
    fi
}

test_linking_links() {
    mktestentry "test"
    mktestentry "test2"

    imag-link internal add --from "test" --to "test2"

    if [[ "$(entry_linked_to '/test')" == "$(cat_entry 'test2')" ]];
    then
        err "Linking to 'test' didn't succeed for 'test2'"
        err $(cat_entry 'test2')
    fi

    if [[ "$(entry_linked_to '/test2')" == "$(cat_entry 'test')" ]];
    then
        err "Linking to 'test2' didn't succeed for 'test'"
        err $(cat_entry 'test')
    fi
}

invoke_tests            \
    test_link_modificates \
    test_linking_links

