#!/bin/sh

rm -rf lib/rivet-cloud/
cp -r ../backend/gen/svc/api-cloud/smithy/rust/ lib/rivet-cloud/
cp -r ../backend/gen/svc/api-cloud/smithy/rust-server/ lib/rivet-cloud-server/

