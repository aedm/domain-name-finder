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

function main() {
  docker login --username aedm --password "${DOCKER_HUB_API_KEY}"

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
}

main