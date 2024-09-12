# Contributing

## Versions

### Publishing Versions

Install the prerequisites:

```bash
cargo install cargo-dist@0.6.2
cargo install cargo-release@0.25.6
```

To release, do the following:

1. Update & commit the version & release date in `CHANGELOG.md`
2. Run: `scripts/release.sh x.x.x`

**Prereleases**

To create a prerelease version, append `rc` like: `x.x.x-rc.x`. `cargo-dist` will automatically flag this as a prerelease on GitHub.

### Retracting a version

If something goes wrong with a deploy:

1. Remove the release on GitHub
2. In order to re-release the version, delete the tag:

    ```
    git tag -d vx.x.x
    git push -u origin :refs/tags/vx.x.x
    ```

## Developing

### Run CLI

```sh
cargo build
./target/debug/rivet --help
```

### Install CLI Globally

To install the 

```sh
cargo install --path packages/cli --debug
```

### Edit Backend Without Rebuild

To iterate faster on the backend without requiring rebuilding the toolchain for every change, run `rivet config edit user` and add this to your config, where `/path/to/toolchain` is the path to this repository:

```json
{
  "backend": {
    "source_path": "/path/to/toolchain/packages/backend/"
  }
}
```

This will direct the toolchain to use the raw source code for the backend instead of the embedded backend source code that requires a rebuild.

## Implementation Notes

### Backend Package

The backend package is a TypeScript package located at `packages/backend/`. This contains:

- `packages/backend/toolchain/` – Library to manage the backend in development
- `packages/backend/cli/` – JSON CLI interface to the toolchain
- `packages/backend/runtime/` – Runtime code used to by the backend at runtime

### Backend Artifacts

Backend artifacts are files that are required to be generated for the backend to run.

Artifacts are automatically generated on Cargo build in `packages/backend-embed/build.rs`. Under the hood, this calls the `scripts/backend/build_artifacts.ts` script.

### Embedding Deno

Deno is automatically downloaded and installed to a temporary path in `packages/deno-embed`. This is used both at runtime for the toolchain and any build scripts that depend on Deno.

### Embedding Backend

The backend is embedded using `include_dir`. This is automatically inflated to the data dir at runtime. The backend path can be overridden using the `backend.source_path` setting.

### Task Architecture

Tasks are structured in a way where the applciation can make a request and receive a stream of events in response until the task completes.

This allows for a simpler architecture by:

- Allowing CLI commands to map nicely to tasks
- Allow a clean callback format for FFIs

