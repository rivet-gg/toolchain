#!/bin/sh
set -euf

mkdir -p ./sdks/rust
rm -rf ./sdks/rust
cp -r $EE_REPO_PATH/sdks/full/rust ./sdks
