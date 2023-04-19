#!/bin/sh

# Copy libraries
rm -rf lib/rivet-cloud/ lib/rivet-api/
cp -r ../backend/gen/svc/api-cloud/smithy/rust/ lib/rivet-cloud/
cp -r ../backend/gen/openapi/internal/rust/ lib/rivet-api/

# HACK: Modify libraries to disallow unknown fields in config
find lib/rivet-api -name "*.rs" -exec sed -i 's/\(#\[derive.*Deserialize.*\]\)/\1\n#[serde(deny_unknown_fields)]/g' {} \;

(cd lib/rivet-cloud && cargo fmt)
(cd lib/rivet-api && cargo fmt)

