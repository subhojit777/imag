#!/usr/bin/env bash

COLOR_OFF='\e[0m'       # Text Reset
RED='\e[0;31m'          # Red
YELLOW='\e[0;33m'       # Yellow
GREEN='\e[0;32m'        # Green

RUNTIME="/tmp"
STORE="${RUNTIME}/store"

out() {
    echo -e "${YELLOW}:: $*${COLOR_OFF}"
}

success() {
    echo -e "${GREEN}>> $*${COLOR_OFF}"
}

err() {
    echo -e "${RED}!! $*${COLOR_OFF}"
}

imag-store() {
    local searchdir=$(dirname ${BASH_SOURCE[0]})/../target/debug/
    [[ -d $searchdir ]] || { err "FATAL: No directory $searchdir"; exit 1; }
    local bin=$(find $searchdir -iname imag-store -type f -executable)
    local flags="--debug --rtp $RUNTIME"
    out "Calling '$bin $flags $*'"
    $bin $flags $*
}

reset_store() {
    rm -r "${STORE}"
}

call_test() {
    out "-- TESTING: '$1' --"
    $1
    result=$?
    if [[ -z "$DONT_RESET_STORE" ]]; then
        out "Reseting store"
        reset_store
        out "Store reset done"
    fi
    [[ $result -eq 0 ]] || { err "-- FAILED: '$1'. Exiting."; exit 1; }
    success "-- SUCCESS: '$1' --"
}

invoke_tests() {
    out "Invoking tests."
    if [[ ! -z "$INVOKE_TEST" ]]; then
        out "Invoking only $INVOKE_TEST"
        call_test "$INVOKE_TEST"
    else
        out "Invoking $*"
        for t in $*; do
            call_test "$t"
        done
    fi
}

