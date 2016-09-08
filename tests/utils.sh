#!/usr/bin/env bash

#
#
# This file contains test utility functions which are used by the test scripts
# for each binary.
#
#


COLOR_OFF='\e[0m'       # Text Reset
RED='\e[0;31m'          # Red
YELLOW='\e[0;33m'       # Yellow
GREEN='\e[0;32m'        # Green

RUNTIME="/tmp"
STORE="${RUNTIME}/store"

out() {
    [[ -z "$DEBUG_OUTPUT_OFF" ]] && echo -e "${YELLOW}:: $*${COLOR_OFF}"
}

success() {
    [[ -z "$DEBUG_OUTPUT_OFF" ]] && echo -e "${GREEN}>> $*${COLOR_OFF}"
}

err() {
    [[ -z "$DEBUG_OUTPUT_OFF" ]] && echo -e "${RED}!! $*${COLOR_OFF}"
}

silent() {
    DEBUG_OUTPUT_OFF=1 $*
}

imag-call-binary() {
    local searchdir=$1; shift
    local binary=$1; shift
    [[ -d $searchdir ]] || { err "FATAL: No directory $searchdir"; exit 1; }
    local bin=$(find $searchdir -iname $binary -type f -executable)
    local flags="--no-color --config ./imagrc.toml --override-config store.implicit-create=true --rtp $RUNTIME"
    out "Calling '$bin $flags $*'"
    $bin $flags $*
}

cat_entry() {
    cat ${STORE}/$1
}

reset_store() {
    rm -rf "${STORE}"/.git
    rm -r "${STORE}"
}

call_test() {
    prepare_store_directory || {
        err "Preparing store directory failed"
        exit 1
    }

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

__git() {
    out "Calling git: $*"
    git --work-tree=/tmp/store/ --git-dir=/tmp/store/.git $*
}

__git_commit() {
    out "Calling git-commit: $*"
    git --work-tree=/tmp/store/ --git-dir=/tmp/store/.git commit -m "$*"
}

prepare_store_directory() {
    out "Preparing /tmp/store"
    mkdir -p /tmp/store/                                &&\
    touch /tmp/store/.gitkeep                           &&\
    __git init                                          &&\
    __git config --local user.email "imag@imag-pim.org" &&\
    __git config --local user.name "Imag CI"            &&\
    __git add .gitkeep                                  &&\
    __git_commit 'Initial commit: .gitkeep'
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


