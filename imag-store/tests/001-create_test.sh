#!/usr/bin/env bash

source $(dirname ${BASH_SOURCE[0]})/utils.sh

test_call() {
    imag-store create -p /test-call~0.1.0
    if [[ ! $? -eq 0 ]]; then
        err "Return value should be zero, was non-zero"
        return 1;
    fi
}

test_call_id() {
    imag-store create -i /test-call~0.1.0
    if [[ ! $? -eq 0 ]]; then
        err "Return value should be zero, was non-zero"
        return 1;
    fi
}

test_call_no_id() {
    imag-store create
    if [[ ! $? -eq 1 ]]; then
        err "Return value should be zero, was non-zero"
        return 1;
    fi
}

test_mkstore() {
    imag-store create -p /test-mkstore~0.1.0 || { err "Calling imag failed"; return 1; }
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

    imag-store create -p /test-std-header~0.1.0
    local result=$(cat ${STORE}/test-std-header~0.1.0)
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

    imag-store create -p /test-std-header-plus-custom~0.1.0 entry -h zzz.zzz=z
    local result=$(cat ${STORE}/test-std-header-plus-custom~0.1.0)
    if [[ "$expected" == "$result" ]]; then
        out "Expected store entry == result"
    else
        err "${STORE}/test differs from expected"
        return 1
    fi
}

test_std_header_plus_custom_multiheader() {
    local expected=$(cat <<EOS
---
[foo]
bar = "baz"

[imag]
links = []
version = "0.1.0"

[zzz]
zzz = "z"
---

EOS
)

    local filename="test-std-header-plus-custom-multiheader~0.1.0"
    imag-store create -p /$filename entry -h zzz.zzz=z foo.bar=baz
    local result=$(cat ${STORE}/$filename)
    if [[ "$expected" == "$result" ]]; then
        out "Expected store entry == result"
    else
        err "${STORE}/$filename differs from expected"
        return 1
    fi
}


test_std_header_plus_custom_multiheader_same_section() {
    local expected=$(cat <<EOS
---
[imag]
links = []
version = "0.1.0"

[zzz]
bar = "baz"
zzz = "z"
---

EOS
)

    local filename="test-std-header-plus-custom-mutliheader-same-section~0.1.0"
    imag-store create -p /$filename entry -h zzz.zzz=z zzz.bar=baz
    local result=$(cat ${STORE}/$filename)
    if [[ "$expected" == "$result" ]]; then
        out "Expected store entry == result"
    else
        err "${STORE}/$filename differs from expected"
        return 1
    fi
}

test_std_header_plus_custom_and_content() {
    local expected=$(cat <<EOS
---
[imag]
links = []
version = "0.1.0"

[zzz]
zzz = "z"
---
content
EOS
)

    local name="test-std-header-plus-custom-and-content~0.1.0"
    imag-store create -p /$name entry -h zzz.zzz=z -c content
    local result=$(cat ${STORE}/$name)
    if [[ "$expected" == "$result" ]]; then
        out "Expected store entry == result"
    else
        err "${STORE}/test differs from expected"
        return 1
    fi
}

invoke_tests                                                \
    test_call                                               \
    test_call_id                                            \
    test_call_no_id                                         \
    test_mkstore                                            \
    test_std_header                                         \
    test_std_header_plus_custom                             \
    test_std_header_plus_custom_multiheader                 \
    test_std_header_plus_custom_multiheader_same_section    \
    test_std_header_plus_custom_and_content

