#!/bin/sh
set -euf

# Copy libraries
rm -rf lib/rivet-api/
cp -r ../backend-ee/oss/gen/openapi/internal/rust/ lib/rivet-api/

# HACK: Modify libraries to disallow unknown fields in config
find lib/rivet-api -name "*.rs" -exec sed -i 's/\(#\[derive.*Deserialize.*\]\)/\1\n#[serde(deny_unknown_fields)]/g' {} \;

(cd lib/rivet-api && cargo fmt)


