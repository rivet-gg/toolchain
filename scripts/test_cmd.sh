#!/bin/sh
set -euf

(cd ../sandbox/ && ../cli/target/debug/rivet "$@")
# (cd ../sandbox && docker run -v "$(pwd):/app" ghcr.io/rivet-gg/cli "$@")

