#!/usr/bin/env bash

# Runs an entire update cycle

set -e
set -x

function check_docker_image() {
  local image=$1

  skopeo --version

  set +e
  skopeo inspect "docker://${image}" 2>/dev/null
  if [ $? -eq 0 ]
  then
    echo "Image '${image}' already exists"
    exit 1
  fi
  set -e

  echo "Image '${image}' doesnt exist yet, continuing..."
}

function upload_zone_file_to_s3() {
  local timestamp=$1
  local filename="./db/zone-file/com-zone-raw.${timestamp}.txt.gz"
  set +e
  aws s3 cp ${filename} s3://domain-com-zone-files --endpoint-url=https://s3.us-west-001.backblazeb2.com
  set -e
}

function main() {
  echo "${DOCKER_HUB_API_KEY}" | docker login --username aedm --password-stdin

  cargo build --package tools --release --bin download_raw_zone_file --manifest-path ./tools/Cargo.toml

  local zone_date=$(cargo run --package tools --release --bin download_raw_zone_file --manifest-path ./tools/Cargo.toml --quiet -- --only-date)
  echo "Zone file date: ${zone_date}"

  local git_hash=$(git rev-parse --short HEAD)
  echo "Git hash: ${git_hash}"

  local tag="${zone_date}-${git_hash}"
  local image="aedm/domain:${tag}"

  check_docker_image $image

  cargo run --package tools --release --bin download_raw_zone_file --manifest-path ./tools/Cargo.toml --quiet
  cargo run --package tools --release --bin convert_raw_zone_to_database --manifest-path ./tools/Cargo.toml --quiet

  echo "Building ${image}"
  docker build -t ${image} .
  docker push ${image}

  upload_zone_file_to_s3 $zone_date
}

main