#!/bin/sh
set -euf

./bin/test-setup.sh
# BUILD_ID="$(./bin/test-cmd.sh build push my-image --name "Local" --format json | jq '.build_id')"
BUILD_ID='"0c09a994-78bd-420e-859e-8c7a096ea942"'
echo "Build ID: $BUILD_ID"
# SITE_ID="$(./bin/test-cmd.sh site push ./dist/public/ --name "Local" --format json | jq '.site_id')"
SITE_ID='"2e1783d9-4f76-4a1b-b9b9-382c6b6b8110"'
echo "Site ID: $SITE_ID"
./bin/test-cmd.sh version read-config -o "cdn.site=$SITE_ID" -o "matchmaker.docker.build=$BUILD_ID"
./bin/test-cmd.sh version create "$(date -u +"%Y-%m-%dT%H:%M:%S")" -o "cdn.site=$SITE_ID" -o "matchmaker.docker.build=$BUILD_ID" --format json 

