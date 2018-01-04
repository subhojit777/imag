#!/usr/bin/env bash
#
# An hook script to check the commit message for style
#
# To enable this hook, copy this file to ".git/hooks/commit-msg" and make it
# executable.

. git-sh-setup


#
#
# Check for "WIP" in commit message and add "[skip ci]" if commit message
# contains a WIP.
#
#

if grep -q -i -e "WIP" -e "work in progress" $1; then
    read -p "You're about to add a WIP commit, do you want to run the CI? [y|n] " -n 1 -r < /dev/tty
    echo
    if echo $REPLY | grep -E '^[Nn]$' > /dev/null; then
        sed -i '1,1s,.*,[ci skip] &,' $1
    fi
fi

