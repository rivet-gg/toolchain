#!/bin/sh
set -euf

(cd ../test-game/ && ../rivetctl/target/debug/rivetctl "$@")
