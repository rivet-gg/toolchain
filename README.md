# Rivet CLI

## Installation

<details>
<summary><b>macOS & Linux & WSL</b></summary>

**Install latest version**

```sh
curl -fsSL https://raw.githubusercontent.com/rivet-gg/cli/main/install/unix.sh | sh
```

**Install specific version**

We recommend specifying the CLI version in CI environments. This also allows you to install prerelease versions of the CLI.

```sh
export RIVET_CLI_VERSION="v1.0.0"
curl -fsSL https://raw.githubusercontent.com/rivet-gg/cli/${RIVET_CLI_VERSION}/install/unix.sh | sh
```

_The `export` keyword is important. The variable `RIVET_CLI_VERSION` needs to be accessible inside the install script._
</details>

<details>
<summary><b>Windows (cmd)</b></summary>

**Install latest version**

```ps1
powershell -Command "iwr https://raw.githubusercontent.com/rivet-gg/cli/main/install/windows.ps1 -useb | iex"
```

**Install specific version**

We recommend specifying the CLI version in CI environments. This also allows you to install prerelease versions of the CLI.

```sh
powershell -Command "$env:RIVET_CLI_VERSION='v1.0.0'; iwr https://raw.githubusercontent.com/rivet-gg/cli/$env:RIVET_CLI_VERSION/install/windows.ps1 -useb | iex"
```
</details>

<details>
<summary><b>Windows (PowerShell)</b></summary>

**Install latest version**

```
iwr https://raw.githubusercontent.com/rivet-gg/cli/main/install/windows.ps1 -useb | iex
```

**Install specific version**

We recommend specifying the CLI version in CI environments. This also allows you to install prerelease versions of the CLI.

```ps1
$env:RIVET_CLI_VERSION='v1.0.0'
iwr https://raw.githubusercontent.com/rivet-gg/cli/$env:RIVET_CLI_VERSION/install/windows.ps1 -useb | iex
```
</details>

<details>
<summary><b>Install from GitHub</b></summary>


```sh
cargo install --git=https://github.com/rivet-gg/cli
```

**Important** This will install the CLI as `rivet-cli` (not `rivet`).
</details>

<details>
<summary><b>Build from source</b></summary>

```sh
git clone https://github.com/rivet-gg/cli
cd cli
cargo build
```

The executable will be available at _target/debug/rivet-cli_.
</details>

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
