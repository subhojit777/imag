#!/usr/bin/env bash

source $(dirname ${BASH_SOURCE[0]})/../../tests/utils.sh
source $(dirname ${BASH_SOURCE[0]})/utils.sh

default_entry() {
    cat <<EOS
---
[imag]
links = []
version = "0.1.0"
---

EOS
}

entry_linked_to() {
    cat <<EOS
---
[imag]
links = [$1]
version = "0.1.0"
---

EOS
}

mktestentry() {
    mkdir -p ${STORE}
    default_entry > ${STORE}/$1
}

test_link_modificates() {
    mktestentry "test~0.1.0"
    mktestentry "test2~0.1.0"

    imag-link internal add --from "test~0.1.0" --to "test2~0.1.0"

    if [[ "$(default_entry)" -eq "$(cat_entry 'test~0.1.0')" ]] ||
       [[ "$(default_entry)" -eq "$(cat_entry 'test2~0.1.0')" ]]
    then
        err "Entry was unmodified after linking"
        return 1;
    fi
}

test_linking_links() {
    mktestentry "test~0.1.0"
    mktestentry "test2~0.1.0"

    imag-link internal add --from "test~0.1.0" --to "test2~0.1.0"

    if [[ "$(entry_linked_to '/test~0.1.0')" == "$(cat_entry 'test2~0.1.0')" ]];
    then
        err "Linking to 'test~0.1.0' didn't succeed for 'test2~0.1.0'"
        err $(cat_entry 'test2~0.1.0')
    fi

    if [[ "$(entry_linked_to '/test2~0.1.0')" == "$(cat_entry 'test~0.1.0')" ]];
    then
        err "Linking to 'test2~0.1.0' didn't succeed for 'test~0.1.0'"
        err $(cat_entry 'test~0.1.0')
    fi
}

invoke_tests            \
    test_link_modificates \
    test_linking_links

