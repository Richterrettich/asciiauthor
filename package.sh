#!/bin/bash
cargo build --release
VERSION=$(cat Cargo.toml | grep version | awk '{gsub(/"/, "", $3);print $3}')
fpm -s dir -t $1 --name asciiauthor --force --version $VERSION target/release/asciiauthor=/usr/bin/asciiauthor
