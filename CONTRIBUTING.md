# Contributing

## Publishing Versions

Install the prerequisites:

```bash
cargo install cargo-dist@0.6.2
cargo install cargo-release@0.25.0
```

To release, do the following:

1. Update & commit the version & release date in `CHANGELOG.md`
2. Run: `scripts/release.sh x.x.x`

**Prereleases**

To create a prerelease version, append `rc` like: `x.x.x-rc.x`. `cargo-dist` will automatically flag this as a prerelease on GitHub.

## Retracting a version

If something goes wrong with a deploy:

1. Remove the release on GitHub
2. In order to re-release the version, delete the tag:

    ```
    git tag -d vx.x.x
    git push -u origin :refs/tags/vx.x.x
    ```

