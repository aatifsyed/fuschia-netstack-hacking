#!/usr/bin/env bash
set -o errexit -o xtrace -o pipefail -o nounset

# See https://fuchsia.googlesource.com/fuchsia/+/refs/heads/main
# when=refs/heads/main
when=ce7e7275a87db712e058487a906a927b2baedac3 # Sat Feb 25 01:04:39 2023 +0000

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