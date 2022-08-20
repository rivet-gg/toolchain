#!/bin/sh
set -euf -o pipefail

(cd ../test-game/ && ../rivetctl/target/debug/rivet "$@")
