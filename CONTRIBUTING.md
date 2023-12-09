# Contributing

## Publishing Versions

Install the prerequisites:

```bash
cargo install cargo-dist@0.5.0
cargo install cargo-release@0.25.0
```

To release, do the following:

1. Update the version in `CHANGELOG.md`
2. Run: `bin/release.sh x.x.x`
