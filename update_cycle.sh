#!/usr/bin/env bash

# Runs an entire update cycle

function main() {
#local zone_date = $(cargo run --package tools --release --bin download_raw_zone_file --manifest-path ./tools/Cargo.toml | grep -Po '^Zone file date "\K.*?(?=")')
local zone_date="20220620-060813"
#cargo run --package tools --release --bin convert_raw_zone_to_database --manifest-path ./tools/Cargo.toml

local git_hash=$(git rev-parse --short HEAD)

echo $zone_date
echo $git_hash

local tag="${zone_date}-${git_hash}"
local image="aedm/domain:${tag}"
echo "Building ${image}"

docker build -t ${image} .
docker push ${image}
}

main