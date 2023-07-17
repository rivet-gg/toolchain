# Contributing

## Publishing Versions

Install the prerequisites:

```bash
cargo install cargo-dist
cargo install cargo-release
```

To release, do the following:

1. Update the version in `CHANGELOG.md`
2. Run: `bin/release.sh x.x.x`
