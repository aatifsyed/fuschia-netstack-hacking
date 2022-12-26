#!/usr/bin/env bash
set -o errexit -o xtrace -o pipefail -o nounset

# See https://fuchsia.googlesource.com/fuchsia/+/refs/heads/main
#when=refs/heads/main
when=ccd71c961977b58cd3cdff0a478795b1d5737519 # Sat Dec 24 15:20:51 2022 +0000

declare -A folder_src_path
folder_src_path=(
    # see also Cargo.toml::workspace::members
    [explicit]=src/connectivity/network/lib/explicit
    [fakestd]=src/connectivity/network/netstack3/core/fakestd
    [net-types]=src/connectivity/lib/net-types
    [packet-formats]=src/connectivity/lib/packet-formats
    [packet]=src/lib/network/packet
)

for folder in "${!folder_src_path[@]}"
do
    mkdir --parents "$folder"
    pushd "$folder"
    find . -not -name 'Cargo.toml' -delete
    curl --fail "https://fuchsia.googlesource.com/fuchsia/+archive/$when/${folder_src_path[$folder]}.tar.gz" \
        | tar --extract --gzip --file - --verbose
    popd
done

curl --fail "https://fuchsia.googlesource.com/fuchsia/+/$when/LICENSE?format=TEXT" \
    | base64 --decode \
    > LICENSE