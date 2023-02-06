#!/bin/sh

rm -rf lib/rivet-cloud/ lib/rivet-api/
cp -r ../backend/gen/svc/api-cloud/smithy/rust/ lib/rivet-cloud/
cp -r ../backend/gen/openapi/rust/ lib/rivet-api/

