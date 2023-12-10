# Rivet CLI

## Installation

### macOS & Linux & WSL

**Install latest version**

```bash
curl -fsSL https://raw.githubusercontent.com/rivet-gg/cli/main/install/unix.sh | sh
```

**Install specific version**

We recommend specifying the CLI version in CI environments.

```bash
export RIVET_CLI_VERSION="v0.2.0"
curl -fsSL https://raw.githubusercontent.com/rivet-gg/cli/${RIVET_CLI_VERSION}/install/unix.sh | sh
```

### Windows (cmd & PowerShell)

**Install latest version**

```
powershell -Command "iwr https://raw.githubusercontent.com/rivet-gg/cli/main/install/windows.ps1 -useb | iex"
```

**Install specific version**

We recommend specifying the CLI version in CI environments.

```bash
powershell -Command "$env:RIVET_CLI_VERSION='v0.2.0'; iwr https://raw.githubusercontent.com/rivet-gg/cli/$env:RIVET_CLI_VERSION/install/windows.ps1 -useb | iex"
```

### Build from source

```
cargo install --git=https://github.com/rivet-gg/cli
```

## Documentation

All commands in the Rivet CLI are documented with the `--help` flag.

```
$ rivet --help

USAGE:
    rivet [OPTIONS] <SUBCOMMAND>

OPTIONS:
        --api-url <API_URL>            [env: RIVET_CLOUD_API_URL=]
        --token <CLOUD_TOKEN>    [env: RIVET_TOKEN=]
    -h, --help                         Print help information
    -V, --version                      Print version information

SUBCOMMANDS:
    dashboard    Opens the dashboard for this game
    deploy       Alias of `rivet version deploy`
    engine       Run engine-specific commands
    game         Manages the game
    help         Print this message or the help of the given subcommand(s)
    image        Manages builds for Serverless Lobbies
    init         Guided setup for this project
    namespace    Manages namespaces
    site         Manages sites for the CDN
    token        Manages tokens
    version      Manages versions
```
