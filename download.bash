#!/usr/bin/env bash
set -o errexit -o xtrace -o pipefail -o nounset

# See https://fuchsia.googlesource.com/fuchsia/+/refs/heads/main
# when=refs/heads/main
when=816672d4d051a2222b63cd3173b3b5893d6ca169 # Fri Feb 02 17:08:57 2024 +0000

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
