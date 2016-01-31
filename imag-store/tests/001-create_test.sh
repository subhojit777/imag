#!/usr/bin/env bash

source $(dirname ${BASH_SOURCE[0]})/utils.sh

test_call() {
    imag-store create -p /test-call
    if [[ ! $? -eq 0 ]]; then
        err "Return value should be zero, was non-zero"
        return 1;
    fi
}

test_mkstore() {
    imag-store create -p /test-mkstore || { err "Calling imag failed"; return 1; }
    if [[ -d ${STORE} ]]; then
        out "Store exists."
    else
        err "No store created"
        return 1
    fi
}

test_std_header() {
    local expected=$(cat <<EOS
---
[imag]
links = []
version = "0.1.0"
---

EOS
)

    imag-store create -p /test-std-header
    local result=$(cat ${STORE}/test-std-header)
    if [[ "$expected" == "$result" ]]; then
        out "Expected store entry == result"
    else
        err "${STORE}/test differs from expected"
        return 1
    fi
}

test_std_header_plus_custom() {
    local expected=$(cat <<EOS
---
[imag]
links = []
version = "0.1.0"

[zzz]
zzz = "z"
---

EOS
)

    imag-store create -p /test-std-header-plus-custom entry -h zzz.zzz=z
    local result=$(cat ${STORE}/test-std-header-plus-custom)
    if [[ "$expected" == "$result" ]]; then
        out "Expected store entry == result"
    else
        err "${STORE}/test differs from expected"
        return 1
    fi
}

invoke_tests                    \
    test_call                   \
    test_mkstore                \
    test_std_header             \
    test_std_header_plus_custom

