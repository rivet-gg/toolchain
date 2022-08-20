#!/bin/sh
set -euf -o pipefail

./bin/test-setup.sh

# MARK: Get game info
GAME_ID="$(./bin/test-cmd.sh game get --format json | jq -r '.game_id')"
echo "Game ID: $GAME_ID"

# MARK: Upload build
BUILD_ID="$(./bin/test-cmd.sh build push my-image --name "Local" --format json | jq -r '.build_id')"
# BUILD_ID='801aed79-d0b7-4ab8-97cb-ca5d4a7c436c'
echo "Build ID: $BUILD_ID"

# MARK: Upload site
# SITE_ID="$(./bin/test-cmd.sh site push ./dist/public/ --name "Local" --format json | jq -r '.site_id')"
SITE_ID='084d24dc-9226-4786-bcb9-48c259bee04f'
echo "Site ID: $SITE_ID"
./bin/test-cmd.sh version read-config -o "cdn.site=\"$SITE_ID\"" -o "matchmaker.docker.build=\"$BUILD_ID\""

# MARK: Create version
VERSION_NAME="$(git rev-parse --abbrev-ref HEAD) $(git rev-parse --short HEAD)"
VERSION_NAME="$(date -u +"%Y-%m-%dT%H:%M:%S")"
VERSION_ID="$(./bin/test-cmd.sh version create --display-name "$VERSION_NAME" -o "cdn.site=\"$SITE_ID\"" -o "matchmaker.docker.build=\"$BUILD_ID\"" --format json | jq -r '.version_id')"
echo "Version ID: $VERSION_ID"

# MARK: Create namespace
# TODO: Impl ignore-existing or something
NS_DISPLAY_NAME="$(git rev-parse --abbrev-ref HEAD)"
NS_NAME_ID="$(sed -E 's/[^[:alnum:]]+/_/g' <<< "$NS_DISPLAY_NAME")"
NAMESPACE_ID="$(./bin/test-cmd.sh namespace create --name-id "$NS_NAME_ID" --display-name "$NS_DISPLAY_NAME"  --version "$VERSION_ID" --format json | jq -r '.namespace_id')"

# MARK: Publish version
./bin/test-cmd.sh namespace set-version --namespace "$NAMESPACE_ID" --version "$VERSION_ID" --format json

./bin/test-cmd.sh game dashboard
./bin/test-cmd.sh namespace dashboard "$NAMESPACE_ID"
./bin/test-cmd.sh version dashboard "$VERSION_ID"

