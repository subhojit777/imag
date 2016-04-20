#!/usr/bin/env bash

source $(dirname ${BASH_SOURCE[0]})/../../tests/utils.sh
source $(dirname ${BASH_SOURCE[0]})/utils.sh

create() {
    imag-store create $*
}

delete() {
    imag-store delete $*
}

test_delete_simple() {
    local name="test~0.1.0"

    create -p /$name
    delete --id /$name

    local n=$($(find ${STORE}/ -type f | wc -l))
    if [[ $n -eq 0 ]]; then
        success "Deleting worked"
    else
        err "There are still $n files in the store"
    fi
}

invoke_tests            \
    test_delete_simple

