#!/usr/bin/env bash
set -o errexit -o xtrace -o pipefail -o nounset

# See https://fuchsia.googlesource.com/fuchsia/+/refs/heads/main
#when=refs/heads/main
when=43f806db60eba587110bc85a505e310d79cae013 # Sun Apr 10 15:37:29 2022 +0000

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

curl --fail "https://fuchsia.googlesource.com/fuchsia/+/$when/LICENSE?format=TEXT" \
    | base64 --decode \
    > LICENSE