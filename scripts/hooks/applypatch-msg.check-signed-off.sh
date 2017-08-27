#!/usr/bin/env bash
#
# An hook script to check the commit log message taken by
# applypatch from an e-mail message for proper "Signed-off-by" line(s).
#
# To enable this hook, copy this file to ".git/hooks/applypatch-msg" and make it
# executable.

#
# This hook is used when applying patches which are send via mail, to verify the
# Signed-off-by line is in the commit message.
#

. git-sh-setup

RED='\e[0;31m'      # Red
YELLOW='\e[0;33m'   # Yellow
NORMAL='\e[0m'      # Text Reset

warn() {
    echo -e >&2 "${YELLOW}$*${DEFAULT}"
}

abort() {
    echo -e >&2 "${RED}$*${DEFAULT}"
    exit 1
}

headline=$(head -n 1 $1 | wc -c)
[[ $headline -gt 50 ]] && warn "Headline of patch longer than 50 chars"

grep "^Signed-off-by" $1 >/dev/null 2>/dev/null && abort "No Signed-off-by line"

