#!/usr/bin/env bash

#
# The following snippet can be used to WARN about "!fixup" / "WIP" / "TMP"
# commits when pushing
#
# Aborting the push is possible
#

remote="$1"
url="$2"

z40=0000000000000000000000000000000000000000

while read local_ref local_sha remote_ref remote_sha
do
    if [ "$local_sha" != $z40 ]
    then
        if [ "$remote_sha" = $z40 ]
        then
            # New branch, examine all commits
            range="$local_sha"
        else
            # Update to existing branch, examine new commits
            range="$remote_sha..$local_sha"
        fi

        # Check for WIP commit
        commit=$(git rev-list -n 1 --grep '^WIP|^TMP|!fixup' "$range")
        if [ -n "$commit" ]
        then
            echo >&2 "Found WIP commit in $local_ref, not pushing"

            # TO NOT ONLY WARN BUT ABORT UNCOMMENT THE NEXT LINE
            # exit 1
        fi

        # Check for commits without sign-off
        if [ "$remote_sha" = $z40 ]; then
            # New branch is pushed, we only want to check commits that are not
            # on master.
            range="$(git merge-base master "$local_sha")..$local_sha"
        fi
        while read ref; do
            msg=$(git log -n 1 --format=%B "$ref")
            if ! grep -q '^Signed-off-by: ' <<<"$msg"; then
                echo >&2 "Unsigned commit $ref"
                exit 1
            fi
        done < <(git rev-list "$range")
        # The process substitution above is a hack to make sure loop runs in
        # the same shell and can actually exit the whole script.
    fi
done

exit 0
