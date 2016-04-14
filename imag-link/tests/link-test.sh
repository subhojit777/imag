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

mktestentry() {
    mkdir -p ${STORE}
    default_entry > ${STORE}/$1
}

test_link_modificates() {
    mktestentry "test~0.1.0"
    mktestentry "test2~0.1.0"

    imag-link --from "test~0.1.0" --to "test2~0.1.0"

    if [[ "$(default_entry)" -eq "$(cat_entry 'test~0.1.0')" ]] ||
       [[ "$(default_entry)" -eq "$(cat_entry 'test2~0.1.0')" ]]
    then
        err "Entry was unmodified after linking"
        return 1;
    fi
}

invoke_tests            \
    test_link_modificates

