# Contributing

## Developing

### Run CLI in Example Project

This is useful for manually testing if commands work.

```sh
cd examples/basic
cargo run --bin rivet -- <COMMAND>
```

To run without rebuilding:

```sh
cd examples/basic
../../target/debug/rivet <COMMAND>
```

### Install CLI Globally

To install the 

```sh
cargo install --path packages/cli --debug
```

### Edit Backend Without Rebuilding Rust Toolchain

To iterate faster on the backend without requiring rebuilding the toolchain for every change, run `rivet config edit user` and add this to your config, where `/path/to/toolchain` is the path to this repository:

```json
{
  "backend": {
    "source_path": "/path/to/toolchain/packages/backend/"
  }
}
```

This will direct the toolchain to use the raw source code for the backend instead of the embedded backend source code that requires a rebuild.

### Backend Helper Deno Tasks

See `packages/backend/deno.jsonc` for extra scripts for the backend.

## Implementation Notes

### Backend Package

The backend package is a TypeScript package located at `packages/backend/`. This contains:

- `packages/backend/toolchain/` – Library to manage the backend in development
- `packages/backend/cli/` – JSON CLI interface to the toolchain
- `packages/backend/runtime/` – Runtime code used to by the backend at runtime

### Embedding Deno

Deno is automatically downloaded and installed to a temporary path in `packages/deno-embed`. This is used both at runtime for the toolchain and any build scripts that depend on Deno.

### Embedding Backend

The backend is embedded using `include_dir`. This is automatically inflated to the data dir at runtime. The backend path can be overridden using the `backend.source_path` setting.

### Task Architecture

Tasks are structured in a way where the applciation can make a request and receive a stream of events in response until the task completes.

This allows for a simpler architecture by:

- Allowing CLI commands to map nicely to tasks
- Allow a clean callback format for FFIs

