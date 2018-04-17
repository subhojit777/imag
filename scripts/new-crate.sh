#!/usr/bin/env bash

# Helper script to create a new crate in the imag workspace

# 1. Creates a new crate
# 2. Adds the required crate meta information
# 3. Sets the version of the crate to the same version as libimagstore
# 4. Adds the crate to the top-level workspace

if [[ "$1" == "-h" || "$1" == "--help" ]];
then
    echo "$0 [bin|lib] ./path/to/new/crate"
    echo
    echo "Execute _only_ from the top level of the repository"
    exit 0
fi

crate_type="$1"
crate_location="$2"

exit_if_empty() {
    [[ -z "$1" ]] && { echo "$2 not passed"; exit 1; }
}

exit_if_empty "$crate_type" "crate type"
exit_if_empty "$crate_location" "crate location"

exists_cmd() {
    command -v $1 || { echo "No $1 found"; exit 1; }
}

exists_cmd "git"
exists_cmd "cargo"

{ cat ./Cargo.toml 2>/dev/null | head -n 1 | grep -q "[workspace]"; } || {
    echo "Not in root of repository as it seems. Exiting";
    exit 1;
}

[[ "$crate_type" == "lib" || "$crate_type" == "bin" ]] || {
    echo "Invalid crate type, use 'lib' or 'bin'";
    exit 1;
}

if [[ -e "$crate_location" ]]; then
    echo "Crate exists: $crate_location"
    exit 1;
fi

IFS=/ read -ra crate_name_parts <<< "$crate_location"
crate_name="${crate_name_parts[-1]}"

if [[ "$crate_type" == "lib" ]];
then
    crate_description="Library for the imag core distribution"
else if [[ "$crate_type" == "bin" ]]; then
    crate_description="Part of the imag core distribution: $crate_name command"
fi

git_name="$(git config user.name)"
git_email="$(git config user.email)"

store="lib/core/libimagstore/Cargo.toml"
crate_version=$(grep -m 1 version $store | cut -d '"' -f 2)

echo "Crate type:        $crate_type"
echo "Crate location:    $crate_location"
echo "Crate name:        $crate_name"
echo "Crate version:     $crate_version"
echo "Crate description: $crate_description"
echo "Crate author:      $git_name <$git_email>"

echo "Not doing anything as this script is not ready yet."
echo "Exiting now"
exit 1

pushd "$(dirname $crate_location)"
crate new --${crate_type} $crate_name

cat <<EOS > ./$crate_name/Cargo.toml
[package]
name = "$crate_name"
version = "$crate_version"
authors = ["$git_name <$git_email>"]

description = "$crate_description"

keywords    = ["imag", "PIM", "personal", "information", "management"]
readme      = "../../../README.md"
license     = "LGPL-2.1"

documentation = "https://imag-pim.org/doc/"
repository    = "https://github.com/matthiasbeyer/imag"
homepage      = "http://imag-pim.org"

[badges]
travis-ci                         = { repository = "matthiasbeyer/imag" }
is-it-maintained-issue-resolution = { repository = "matthiasbeyer/imag" }
is-it-maintained-open-issues      = { repository = "matthiasbeyer/imag" }
maintenance                       = { status     = "actively-developed" }

[dependencies]

EOS

echo "Cargo.toml written. Please make sure that the README has the right path!"
popd

git add ${crate_location}/*

sed -i "$ s/]/    \"${crate_location}\",\n]/" Cargo.toml
echo "Top-level Cargo.toml modified. Please sort crate list manually!"

