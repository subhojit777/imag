#!/usr/bin/env bash

# Checks whether all commit between $1 and $2 have a signed-off-by line

RED='\e[0;31m' # Red
NORMAL='\e[0m' # Text Reset

faulty=$(git rev-list --grep "Signed-off-by" --invert-grep $1..$2 | wc -l)

if [[ $faulty -eq 0 ]]
then
    echo >&2 "All good"
else
    echo -en >&2 "${RED}Got $faulty non Signed-off-by commits${NORMAL}"
    echo -e  >&2 "${RED}between $1 and $2${NORMAL}"
fi

