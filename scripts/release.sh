#!/bin/sh
set -euf

cargo release --package rivet-cli --execute --no-publish --tag-prefix "" "$1"

