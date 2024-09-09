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
<summary><b>Build from source (<code>cargo install</code>)</b></summary>


```sh
cargo install --git=https://github.com/rivet-gg/toolchain rivet-cli
```
</details>

<details>
<summary><b>Build from source (<code>cargo build</code>)</b></summary>

```sh
git clone https://github.com/rivet-gg/toolchain
cd packages/cli
cargo build
```

The executable will be available at _target/debug/rivet_.
</details>

## Documentation

All commands in the Rivet CLI are documented with the `--help` flag.

```
$ rivet --help

Usage: rivet <COMMAND>

Commands:
  init     Login to a game
  login    Login to a game
  logout   Logout from a game
  dev      Run the development server
  deploy   Build & upload the game server & backend
  config   Manage Rivet configuration
  clean    Remove artifacts that Rivet generates
  create   Add functionality to backend
  db       Manage Postgres database
  sdk      Manage the Rivet SDK
  backend  Manage the backend
  module
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
