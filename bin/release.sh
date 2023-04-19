#!/bin/sh
set -euf -o pipefail

cargo release --execute --no-publish --package rivet-cli "$1"

