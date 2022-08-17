#!/bin/sh
set -euf

./bin/test-setup.sh
./bin/test-cmd.sh "$@"

