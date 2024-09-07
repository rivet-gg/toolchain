#!/bin/sh
set -euf

./scripts/test_setup.sh
./scripts/test_cmd.sh "$@"

