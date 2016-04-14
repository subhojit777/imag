#!/usr/bin/env bash

source $(dirname ${BASH_SOURCE[0]})/../../tests/utils.sh
source $(dirname ${BASH_SOURCE[0]})/utils.sh

std_header() {
    cat <<EOS
---
[imag]
links = []
version = "0.1.0"
---
EOS
}

retrieve() {
    silent imag-store retrieve $*
}

test_retrieve_nothing() {
    local id="test-retrieve_nothing~0.1.0"

    imag-store create -p /${id} || { err "create failed"; return 1; }

    out "Going to test the retrieve functionality now"
    local zero_out="$(retrieve --id /${id})"
    out "Retrieving for zero_out finished"

    if [[ ! -z "$zero_out" ]]; then
        err "Expected zero output, got '$zero_out'"
        return 1
    fi
}

test_retrieve_content() {
    local id="test-retrieve_simple~0.1.0"

    imag-store create -p /${id} || { err "create failed"; return 1; }

    out "Going to test the retrieve functionality now"

    local content_out="$(retrieve --id /${id} --content)"
    out "Retrieving for content_out finished"

    if [[ ! -z "$content_out" ]]; then
        err "Expected content output == zero output, got '$content_out'"
        return 1
    fi
}

test_retrieve_header() {
    local id="test-retrieve_simple~0.1.0"

    imag-store create -p /${id} || { err "create failed"; return 1; }

    out "Going to test the retrieve functionality now"
    local header_out="$(retrieve --id /${id} --header)"
    out "Retrieving for header_out finished"

    if [[ ! "$header_out" != "$(std_header)" ]]; then
        err "Expected header as output, got '$header_out'"
        return 1
    fi
}

test_retrieve_raw() {
    local id="test-retrieve_simple~0.1.0"

    imag-store create -p /${id} || { err "create failed"; return 1; }

    out "Going to test the retrieve functionality now"
    local both_out="$(retrieve --id /${id} --raw)"
    out "Retrieving for both_out finished"

    if [[ "$both_out" != "$(std_header)" ]]; then
        err "Expected "$(std_header)" as output, got '$both_out'"
        return 1
    fi
}

invoke_tests                            \
    test_retrieve_nothing               \
    test_retrieve_content               \
    test_retrieve_header                \
    test_retrieve_raw
