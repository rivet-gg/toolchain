#!/bin/sh
set -euf -o pipefail

(cd ../test-game/ && ../cli/target/debug/rivet "$@")
