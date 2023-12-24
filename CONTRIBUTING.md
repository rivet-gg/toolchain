# Contributing

## Publishing Versions

Install the prerequisites:

```bash
cargo install cargo-dist@0.6.2
cargo install cargo-release@0.25.0
```

To release, do the following:

1. Update the version & release date in `CHANGELOG.md`
2. Run: `scripts/release.sh x.x.x`

To create a prerelease version, append `rc` like: `x.x.x-rc.x`. `cargo-dist` will automatically flag this as a prerelease on GitHub.

