#!/bin/bash

if [ "$1" == "" ]; then
    echo "Usage: version.sh X.Y.Z"
    exit 1
fi

cd ./palette
current_version="$(cargo read-manifest | sed 's/.*"version":"\([^"]\+\)".*/\1/g')"
current_short_version="$(echo "$current_version" | sed 's/\([^.]\+\.[^.]\+\)\..*/\1/g')"
new_short_version="$(echo "$1" | sed 's/\([^.]\+\.[^.]\+\)\..*/\1/g')"
cd ..

echo "updating from $current_version to $1"

sed -i 's/\[Released\](https:\/\/docs.rs\/palette\/'$current_version'\/palette\/)/[Released](https:\/\/docs.rs\/palette\/'$1'\/palette\/)/' README.md
sed -i 's/palette = "'$current_short_version'"/palette = "'$new_short_version'"/' README.md

for path in ./palette/ ./palette_*/; do
    cd $path
    crate="$(echo "$path" | sed 's/\.\/\(.*\)\//\1/g')"
    echo updating $crate
    sed -i 's/version = "'$current_version'" #automatically updated/version = "'$1'" #automatically updated/' Cargo.toml

    sed -i 's/documentation\s*=\s*"https:\/\/docs.rs\/palette\/'$current_version'\/palette\/"/documentation = "https:\/\/docs.rs\/palette\/'$1'\/palette\/"/' Cargo.toml
    sed -i 's/documentation\s*=\s*"https:\/\/docs.rs\/'$crate'\/'$current_version'\/'$crate'\/"/documentation = "https:\/\/docs.rs\/'$crate'\/'$1'\/'$crate'\/"/' Cargo.toml
    sed -i 's/palette_\(.*\)=\s*{version = "'$current_version'"\(.*\)$/palette_\1= {version = "'$1'"\2/' Cargo.toml

    sed -i 's/#!\[doc(html_root_url\s*=\s*"https:\/\/docs.rs\/palette\/'$current_version'\/palette\/"/#![doc(html_root_url = "https:\/\/docs.rs\/palette\/'$1'\/palette\/"/' src/lib.rs
    sed -i 's/#!\[doc(html_root_url\s*=\s*"https:\/\/docs.rs\/'$crate'\/'$current_version'\/'$crate'\/"/#![doc(html_root_url = "https:\/\/docs.rs\/'$crate'\/'$1'\/'$crate'\/"/' src/lib.rs
    cd ..
done

echo updating CHANGELOG.md
bash scripts/changelog.sh
