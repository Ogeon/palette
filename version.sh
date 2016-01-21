#!/bin/bash

if [ "$1" == "" ]; then
    echo "Usage: version.sh X.Y.Z"
    exit 1
fi

current_version="$(cargo read-manifest | sed 's/.*"version":"\([^"]\+\)".*/\1/g')"
current_short_version="$(echo "$current_version" | sed 's/\([^.]\+\.[^.]\+\)\..*/\1/g')"
new_short_version="$(echo "$1" | sed 's/\([^.]\+\.[^.]\+\)\..*/\1/g')"

echo "updating from $current_version to $1"

sed -i 's/version = "'$current_version'" #automatically updated/version = "'$1'" #automatically updated/' Cargo.toml
sed -i 's/palette = "'$current_short_version'"/palette = "'$new_short_version'"/' README.md

bash scripts/changelog.sh
