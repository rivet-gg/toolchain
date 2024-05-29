#!/bin/sh
set -euf

# To do a release, first, merge the Release PR created by release-please. You
# should be able to see the open PR at this link:
#
# https://github.com/rivet-gg/cli/labels/autorelease%3A%20pending
#
# Once the PR is merged, you can run this script with the version that was shown
# in the PR title. For example, the following PR was 1.2.0:
#
# https://github.com/rivet-gg/cli/pull/245
#
# So you would run:
# ./scripts/release.sh 1.2.0
#
# That will run CI that will create the release. This will also run a job that
# will change the label of the release-please PR, since it's not done
# automatically. After the release is created, you'll need to copy items from
# the changelog into the release description:
#
# https://github.com/rivet-gg/cli/releases/edit/v1.1.0
cargo release --package rivet-cli --execute --no-publish --tag-prefix "" "$1"
