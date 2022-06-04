#!/usr/bin/env bash

RUSTFLAGS="-A dead_code -A unused_variables -A unused_mut -A unused_imports" cargo run --package tools --release --bin download_raw_zone_file --manifest-path ./tools/Cargo.toml
