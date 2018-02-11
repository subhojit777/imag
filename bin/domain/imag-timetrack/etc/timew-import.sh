#!/usr/bin/env bash

# timewarrior import script
#
# pipe `timew export` to this script for importing timew data into imag.
#
# Requirements for running this script:
# - imag
# - imag-timetrack
# - sed
# - jq
#

# This might be hacky, but it works.

fixtimeformat() {
    sed -E 's/([0-9]{4})([0-9]{2})([0-9]{2})T([0-9]{2})([0-9]{2})([0-9]{2})Z/\1-\2-\3T\4:\5:\6/'
}

tail -n +2 | head -n -2 | while read line; do
json=$(echo "$line" | sed 's/,$//')
start=$(echo "$json" | jq '.start' | fixtimeformat | sed 's/"//g' )
    end=$(echo "$json" | jq '.end'  | fixtimeformat | sed 's/"//g' )
    tags=$(echo "$json" | jq '.tags' | grep "\"" | sed 's/,//; s/"//g')

    echo imag timetrack track "$start" "$end" $tags
    imag timetrack track "$start" "$end" $tags
done

