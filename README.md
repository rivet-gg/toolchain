# Rivet CLI

## Installing

### macOS & Linux & WSL

```
curl -fsSL https://raw.githubusercontent.com/rivet-gg/cli/main/install/unix.sh | sh
```

### Windows (cmd & PowerShell)

```
powershell -Command "iwr https://raw.githubusercontent.com/rivet-gg/cli/main/install/windows.ps1 -useb | iex"
```

## Publishing Versions


1. Update the version in `CHANGELOG.md`
2. Run the following command

```bash
# One-time setup
cargo install cargo-dist
cargo install cargo-release

# Release a version
cargo release X.X.X
```

