#!/bin/sh

echo "WARNING: Only implemented for macOS x86"

VERSION="v2.0.0-rc.4"

cargo build
cp target/debug/rivet "/Users/nathan/.rivet/${VERSION}/bin/rivet-cli-aarch64-mac"

