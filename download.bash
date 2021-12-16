#!/usr/bin/env bash
set -o errexit -o xtrace -o pipefail

#when=refs/heads/main
when=34f9f202bfc376baf693ae652e0af6036b64f5b7

declare -A folder_src_path
folder_src_path=(
    [explicit]=src/connectivity/network/lib/explicit
    [fakestd]=src/connectivity/network/netstack3/core/fakestd
    [net-types]=src/connectivity/lib/net-types
    [packet-formats]=src/connectivity/lib/packet-formats
    [packet]=src/lib/network/packet
)

for folder in "${!folder_src_path[@]}"
do
    curl --fail "https://fuchsia.googlesource.com/fuchsia/+archive/$when/${folder_src_path[$folder]}.tar.gz" \
        | tar --extract --gzip --file - --verbose --directory "$folder"
done