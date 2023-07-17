#!/bin/sh
set -euf -o pipefail

(cd ../test-game/ && ../cli/target/debug/rivet "$@")
# (cd ../test-game && docker run -v "$(pwd):/app" ghcr.io/rivet-gg/cli "$@")

