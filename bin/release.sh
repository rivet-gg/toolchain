#!/bin/sh
set -euf -o pipefail

cargo release --package rivet-cli --execute --no-publish --tag-prefix "" "$1"

