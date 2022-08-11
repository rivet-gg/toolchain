#!/bin/sh
set -euf

cargo build
(cd ../test-game/ && ../rivetctl/target/debug/rivetctl "$@")

