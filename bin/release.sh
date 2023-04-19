#!/bin/sh
set -euf -o pipefail

cargo release --execute --no-publish --package rivet-cli --package rivet-cli-core "$1"

