#!/bin/sh
set -euf -o pipefail

./bin/test-setup.sh
./bin/test-cmd.sh "$@"

