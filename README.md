# Rivet CLI

## Installation

### macOS & Linux & WSL

```
curl -fsSL https://raw.githubusercontent.com/rivet-gg/cli/main/install/unix.sh | sh
```

### Windows (cmd & PowerShell)

```
powershell -Command "iwr https://raw.githubusercontent.com/rivet-gg/cli/main/install/windows.ps1 -useb | iex"
```

## Documentation


## Publishing Versions

Install the prerequisites:

```bash
cargo install cargo-dist
cargo install cargo-release
```

To release, do the following:

1. Update the version in `CHANGELOG.md`
2. Run: `bin/release.sh x.x.x`

