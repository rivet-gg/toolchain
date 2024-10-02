# Rivet Toolchain

## Installation

<details>
<summary><b>Windows (x86)</b></summary>

```sh
curl -L https://releases.rivet.gg/toolchain/v2.0.0-rc.5/windows_x86_64/rivet.exe.zip -o rivet.exe.zip
unzip rivet.exe.zip
```

Add the directory containing rivet.exe to your PATH environment variable.
</details>

<details>
<summary><b>macOS (Apple Silicon)</b></summary>

```sh
curl -L https://releases.rivet.gg/toolchain/v2.0.0-rc.5/macos_arm64/rivet.zip -o rivet.zip
unzip rivet.zip
chmod +x rivet
sudo mv rivet /usr/local/bin/
```
</details>

<details>
<summary><b>macOS (Intel)</b></summary>

```sh
curl -L https://releases.rivet.gg/toolchain/v2.0.0-rc.5/macos_x86_64/rivet.zip -o rivet.zip
unzip rivet.zip
chmod +x rivet
sudo mv rivet /usr/local/bin/
```
</details>

<details>
<summary><b>Linux (x86)</b></summary>

```sh
curl -L https://releases.rivet.gg/toolchain/v2.0.0-rc.5/linux_x86_64/rivet.zip -o rivet.zip
unzip rivet.zip
chmod +x rivet
sudo mv rivet /usr/local/bin/
```
</details>

<details>
<summary><b>Build from source (`cargo install`)</b></summary>

1. [Install Rust](https://rustup.sh)
2. Install Rivet

```sh
cargo install --git=https://github.com/rivet-gg/toolchain rivet-cli
```
</details>

<details>
<summary><b>Build from source (`cargo build`)</b></summary>

1. [Install Rust](https://rustup.sh)
2. Install Rivet

```sh
git clone https://github.com/rivet-gg/toolchain.git
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
