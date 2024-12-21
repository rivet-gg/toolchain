# Archived Repository

⚠️ **This repository is archived and no longer maintained. It has been moved to [rivet-gg/rivet](https://github.com/rivet-gg/rivet).** ⚠️

# ⛓️ Rivet Toolchain

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

## Quick Start

After installation, you can quickly get started with Rivet by following these steps:

1. Initialize a new Rivet project:
   ```sh
   rivet init
   ```

2. Start the development server:
   ```sh
   rivet dev
   ```

3. Deploy your game:
   ```sh
   rivet login
   rivet deploy
   ```
## Troubleshooting

If you encounter any issues while using Rivet, try the following:

1. Ensure you're using the latest version of Rivet
2. Use the `rivet clean` command to remove any artifacts that might be causing problems
3. Join our [Discord](https://rivet.gg/discord) for support

Please [open an issue](https://github.com/rivet-gg/toolchain/issues) for any issues you run in to.

## Contributing

We welcome contributions to the Rivet Toolchain! If you'd like to contribute, please:

1. Fork the repository
2. Create a new branch for your feature
3. Make your changes
4. Submit a pull request

For more detailed information, see our [contribution guidelines](CONTRIBUTING.md).

## License

Rivet Toolchain is released under the [Apache 2.0 License](LICENSE).
