#!/usr/bin/env bash
#
# A helper script for the mutt query_command
#
# Use
#
#     query_command = "/path/to/this/script %s"
#
# in mutt to search contacts from mutt with imag.
#

# mutt wants a one-line status message before the list of contacts.
echo "Searching with imag ..."
imag contact find "$1" --json | \
jq --raw-output '
    map(.fullname[0] as $FN
    | .email
    | map({email: ., fullname: $FN}))
    | flatten
    | map([.email.address, .fullname, .email.properties.TYPE])
    | .[]
    | @tsv
'
