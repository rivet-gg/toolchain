# Contributing

## Publishing Versions

Install the prerequisites:

```bash
cargo install cargo-dist
cargo install cargo-release
```

To release, do the following:

1. Update the version & release date in `CHANGELOG.md`
2. Run: `scripts/release.sh x.x.x`
