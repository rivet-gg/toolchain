# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.3.3](https://github.com/rivet-gg/cli/compare/v1.3.2...v1.3.3) (2024-07-10)


### Features

* add --skip-migrate flag to opengb deploy ([#262](https://github.com/rivet-gg/cli/issues/262)) ([6255e07](https://github.com/rivet-gg/cli/commit/6255e0770dcea42d97b26357559fd0672257e675))


### Bug Fixes

* **opengb:** one db per env ([#256](https://github.com/rivet-gg/cli/issues/256)) ([a3c4e10](https://github.com/rivet-gg/cli/commit/a3c4e109d6ec316fc72c296b97efa4ef1aef11f9))
* revert regression with config-rs dependency ([#270](https://github.com/rivet-gg/cli/issues/270)) ([2dbb0b9](https://github.com/rivet-gg/cli/commit/2dbb0b9871e3bd8e342ab48f08dc8104677df1b7))


### Chores

* release 1.3.3 ([aae8e3b](https://github.com/rivet-gg/cli/commit/aae8e3bd2e58baa4fc75888b40ecf1a194077205))
* update posthog api key ([#263](https://github.com/rivet-gg/cli/issues/263)) ([5f2ee58](https://github.com/rivet-gg/cli/commit/5f2ee58a5b5bb204dd7d4178446bb1de304a9c0f))

## [1.3.2](https://github.com/rivet-gg/cli/compare/v1.3.1...v1.3.2) (2024-06-13)


### Features

* run opengb using docker by default ([#254](https://github.com/rivet-gg/cli/issues/254)) ([598ce8d](https://github.com/rivet-gg/cli/commit/598ce8da485ac035a834ab74bae33701b34af226))


### Chores

* release 1.3.2 ([50bd2be](https://github.com/rivet-gg/cli/commit/50bd2be003cf8561f5d444a6abd611e3f1078af5))

## [1.3.1](https://github.com/rivet-gg/cli/compare/v1.3.0...v1.3.1) (2024-06-10)


### Continuous Integration

* change label update to merged PRs ([#249](https://github.com/rivet-gg/cli/issues/249)) ([06a938c](https://github.com/rivet-gg/cli/commit/06a938cafedb2ed794358b0a9a370453fd2859dd))


### Chores

* fmt sdk ([#251](https://github.com/rivet-gg/cli/issues/251)) ([28eade7](https://github.com/rivet-gg/cli/commit/28eade7f45a032f08d3fb9c55ddc3b3eda3a1be5))
* **opengb:** migrate from backend.yaml -&gt; backend.json ([#253](https://github.com/rivet-gg/cli/issues/253)) ([4b31887](https://github.com/rivet-gg/cli/commit/4b31887bd166e71958155b8cc5a75bc8246b6248))
* **readme:** add note about openssl when building from source ([#234](https://github.com/rivet-gg/cli/issues/234)) ([a9c1b29](https://github.com/rivet-gg/cli/commit/a9c1b295a4819fb88f9be5a0e52780d5ab92bf27))

## [1.3.0](https://github.com/rivet-gg/cli/compare/v1.2.0...v1.3.0) (2024-05-29)


### Features

* get lobby and logs links in sidekick ([#235](https://github.com/rivet-gg/cli/issues/235)) ([7c63efd](https://github.com/rivet-gg/cli/commit/7c63efd86a2ca4a659b0df8c88b3764f904f2938))


### Bug Fixes

* prevent asking user for terminal permissions ([#236](https://github.com/rivet-gg/cli/issues/236)) ([a1a75d8](https://github.com/rivet-gg/cli/commit/a1a75d858a99ab3830ed14917a26e8f06f446c4f))
* read_generated_manifest fn name ([#241](https://github.com/rivet-gg/cli/issues/241)) ([72970c7](https://github.com/rivet-gg/cli/commit/72970c7240f1dfa19a4fb75a9e009e8bde3799b5))
* reading byte-order marks on Windows ([#238](https://github.com/rivet-gg/cli/issues/238)) ([e177ad4](https://github.com/rivet-gg/cli/commit/e177ad4917945f6c99b8cd2f03c35bec3ba91941))


### Documentation

* release script instructions ([#248](https://github.com/rivet-gg/cli/issues/248)) ([0d9edb3](https://github.com/rivet-gg/cli/commit/0d9edb3737989709ad9d3221d13c5471f997e6e2))


### Continuous Integration

* and release please pr ([#244](https://github.com/rivet-gg/cli/issues/244)) ([9862c5a](https://github.com/rivet-gg/cli/commit/9862c5ada4f935d64cc457d0ecd760a6d7d252b0))
* change release-please pr labels on release ([#247](https://github.com/rivet-gg/cli/issues/247)) ([336f789](https://github.com/rivet-gg/cli/commit/336f789b3909392fe92180ba75382f12d005c8de))
* explicitly fmt check members ([#242](https://github.com/rivet-gg/cli/issues/242)) ([f14b17e](https://github.com/rivet-gg/cli/commit/f14b17ed23a33b34f738975489a92d431dae1c59))
* ignore failing e2e test ([#243](https://github.com/rivet-gg/cli/issues/243)) ([242e291](https://github.com/rivet-gg/cli/commit/242e291db0febec9f70632536d1da39c1293ff30))


### Chores

* Bump the cargo group across 1 directory with 4 updates ([#228](https://github.com/rivet-gg/cli/issues/228)) ([a192e35](https://github.com/rivet-gg/cli/commit/a192e35aa5d5076be10d0f3b23836cfcc28ad1b0))
* **main:** release 1.2.0 ([555fec1](https://github.com/rivet-gg/cli/commit/555fec1a2e1dacc08bc03cbfaed733f146d06220))

## [1.2.0](https://github.com/rivet-gg/cli/compare/v1.1.0...v1.2.0) (2024-05-28)


### Features

* add opengb db command passthrough ([#216](https://github.com/rivet-gg/cli/issues/216)) ([7b78870](https://github.com/rivet-gg/cli/commit/7b788705687bd98387380e785614dbcc8c1190dd))
* add passthrough env var ([#231](https://github.com/rivet-gg/cli/issues/231)) ([2fc3021](https://github.com/rivet-gg/cli/commit/2fc30210e63e0230f88c9a7e04b54a66bb385fab))
* add support for sh and url db commands ([#217](https://github.com/rivet-gg/cli/issues/217)) ([bbeeaba](https://github.com/rivet-gg/cli/commit/bbeeaba7245839047c02f1f461869ab8c434e0ba))
* Implement OpenGB related commands ([#215](https://github.com/rivet-gg/cli/issues/215)) ([ce57364](https://github.com/rivet-gg/cli/commit/ce57364d138d80ea48902733df1b3f796d51cd05))


### Bug Fixes

* add concurrency constraint to generated github action ([#226](https://github.com/rivet-gg/cli/issues/226)) ([8a62d97](https://github.com/rivet-gg/cli/commit/8a62d97bcea701983df02502f801d4ca8f403eef))
* **backend:** check opengb and deno installation using which crate ([#237](https://github.com/rivet-gg/cli/issues/237)) ([64b3489](https://github.com/rivet-gg/cli/commit/64b3489f61206f58299cff59a5583c45b4663bac))
* **ci:** update ci script to use json-compact instead of json ([#224](https://github.com/rivet-gg/cli/issues/224)) ([2f04ea3](https://github.com/rivet-gg/cli/commit/2f04ea3c0639065a10f4b2ecbf4cfc2bf587f353))
* read_generated_manifest fn name ([#241](https://github.com/rivet-gg/cli/issues/241)) ([72970c7](https://github.com/rivet-gg/cli/commit/72970c7240f1dfa19a4fb75a9e009e8bde3799b5))
* update sdks for opengb ([#233](https://github.com/rivet-gg/cli/issues/233)) ([7feb70b](https://github.com/rivet-gg/cli/commit/7feb70b2056d96ac31a69102d8a172ad6c0e0905))
* **upload:** increase upload buffer size ([#229](https://github.com/rivet-gg/cli/issues/229)) ([28d9d93](https://github.com/rivet-gg/cli/commit/28d9d93a9e7d6df959fa2a731c7433febfbe47b0))


### Continuous Integration

* and release please pr ([#244](https://github.com/rivet-gg/cli/issues/244)) ([9862c5a](https://github.com/rivet-gg/cli/commit/9862c5ada4f935d64cc457d0ecd760a6d7d252b0))
* explicitly fmt check members ([#242](https://github.com/rivet-gg/cli/issues/242)) ([f14b17e](https://github.com/rivet-gg/cli/commit/f14b17ed23a33b34f738975489a92d431dae1c59))
* ignore failing e2e test ([#243](https://github.com/rivet-gg/cli/issues/243)) ([242e291](https://github.com/rivet-gg/cli/commit/242e291db0febec9f70632536d1da39c1293ff30))


### Chores

* Release ([b10bc24](https://github.com/rivet-gg/cli/commit/b10bc2414434bc1a93690ea2948feb52003f4bcd))

## [v1.1.0] - 2024-04-13

### Added

- `rivet run` and `rivet exec` are no longer experimental

### Changed

- Rename `--rivet-servers` to `--servers` and `--this-machine` to `--dev` for `rivet run` and `rivet exec`

### Fixed

- `rivet exec` does not respect `--rivet-servers` flag

## [v1.0.2] - 2024-02-29

### Changed

- Progress bars will consolidate to 1 if there are more than 40 files being uploaded
- Update SDKs

### Fixed

- `cdn.build_env` not being passed to `cdn.build_cmd`

## [v1.0.1] - 2024-01-29

### Changed

- Improved progress indicators on file uploads

### Fixed

- Docker image UID & GID validation not getting ran
- Lack of a newline printed by `rivet token create` causing EOL mark to appear on zsh shells

## [v1.0.0] - 2024-01-23

## [v1.0.0-rc.3] - 2024-01-19

### Added

- Shorthand API endpoints can now be passed without the scheme (e.g. `api.mydomain.com` or `127.0.0.1:8080`)
- `rivet global-config read-project` command
- `rivet global-config path` command to get the path to the global config
- `--format` now supports `json-compact`

### Changed

- `--format json` now defaults to pretty-printed JSON

### Fixed

- `rivet unlink` now works even if the credentials are invalid
- Docker image UID & GID validation no longer disabled by default

## [v1.0.0-rc.2] - 2024-01-13

### Added

- `rivet exec` command to run arbitrary commands with `RIVET_API_ENDPOINT` and `RIVET_TOKEN` environment variables
- `rivet run` command to run scripts from the `scripts` portion of `rivet.yaml` with `RIVET_API_ENDPOINT`, `RIVET_TOKEN`, and `RIVET_NAMESPACE` environment variables
- `rivet deploy` now can now specify the namespace inline (e.g. `rivet deploy prod` instead of `rivet deploy -n prod`)
- `matchmaker.docker.build_args` to configure Docker build args
- `cdn.build_env` to configure environment variables for building the site
- `RIVET_API_ENDPOINT` and `RIVET_NAMESPACE` arg is passed to `docker build` by default
- `RIVET_TOKEN` and `RIVET_NAMESPACE` now additionally passed to `cdn.build_command`

### Changed

- Reworked `rivet init` process to cleanly communicate next steps & unique links for the selected engine
- Updated generated `rivet.yaml` on `rivet init` to be more concise and helpful & unique content for the selected engine
- Update OCI bundle archival process to operate on TAR streams instead of using the host's file system to preserve ownership & permissions
- **[BREAKING]** `rivet deploy` now requires a `--no-namespace` flag if no namespace is provided

### Fixed

- Overriding `matchmaker.docker.image_id` getting ignored
- `rivet config validate` now uses `--print` flag instead of a positional argument
- Validate Docker images do not run as GID 0

## [v1.0.0-rc.1] - 2023-12-24

### Added

- Add `x86_64-unknown-linux-musl` artifact
- Version names are now generated with incrementing indexes on the backend without race conditions
- Warning if running unauthenticated commands as a sudo user
- `sidekick unlink` subcommand to unlink the current project from the
  Rivet CLI
- `sidekick generate-config` subcommand to generate a Rivet config file
- `sidekick get-namespace-dev-token` and `sidekick
get-namespace-public-token` subcommands to get a Rivet token for a namespace
- `sidekick get-bootstrap-data` subcommand to get the initial data about
  the signed-in user
- `sidekick get-cli-version` subcommand to get the version of the Rivet
  CLI
- `sidekick deploy` to do the process of deploying a build to Rivet
- ability for `sidekick` to open terminal in a new window for commands
  that need to be shown (e.g. `sidekick deploy`)
- `sidekick get-version` subcommand to get the manage version URL in the
  hub
- `sidekick get-token` subcommand to get a Rivet token for a user
- `sidekick check-login-state` subcommand to see if a user is logged in
  through the CLI
- `sidekick wait-for-login` subcommand to long-poll for a user to sign in
- `sidekick get-link` subcommand to get a sign-in link for a user
- hidden `Sidekick` subcommand to be used by external tools (e.g. engine
  plugins) to interact with the Rivet CLI

### Changed

- Cleaner unauthenticated error
- Changed `sidekick` to a more modular architecture
- Changed error handling in CLI to use `GlobalResult` from main repo instead of
  `anyhow`
- Unix install script can now take the environment variable `BIN_DIR` to specify
  the installation directory, preventing the need for sudo in certain cases
- Rivet CLI now references the `rivet-cli-api` from the Rivet main repo rather
  than storing its own copy
- Update `cargo-dist` to 0.6.2

### Fixed

- Custom engines no longer get prompted to select engine when running `rivet init` for the second time
- Windows compilation no longer fails with `nix` dependency
- `--telemetry-disabled` no longer requires explicit `true`
- Collect system metrics using `sysinfo::System` instead of `uname` command for compatability with Windows
- CDN URL on deploy complete now pulls dynamic DNS from bootstrap API
- CDN URL on deploy complete is no longer displayed if CDN is not enabled for the game

## [v0.4.0] - 2023-12-20

### Added

- Auto-generate GitHub Actions with `rivet ci generate github`
- Development token cache to make `rivet token create development` run faster
- Shorthand `-n` for `--namespace` flag in `rivet token create development`
- `rivet deploy` validates config before building & uploading resources
- `rivet unlink` command to remove authentication token
- Pretty-printed errors instead of default debug format
- Error reporting to Sentry

### Changed

- Removed engine prompt if Rivet config already exists
- **[BREAKING]** No longer automatically creates/updates `.env` file in favor of using `rivet token create development`
- Global flags (`--api-endpoint`, `--token`, and `--disable-telemetry`) can now be used in subcommands (e.g. `rivet init --token foobar` instead of `rivet --token foobar init`)
- Moved project metadata to global configuration file
- Removed `.rivet` from auto-generated `.gitignore`
- `rivet namespace create` can be called without specifying `--version`
- **[BREAKING]** Change `TELEMETRY_DISABLED` env var to `RIVET_TELEMETRY_DISABLED`
- Remove trailing line break from `rivet token create development`
- Rename `rivet site` subcommands to `rivet cdn` (alias still supported)
- Rename `rivet image` subcommands to `rivet docker` (alias still supported)
- Rename `dashboard` subcommands to `view` (alias still supported)
- Move `rivet version deploy` to `rivet deploy`
- Move `rivet version config-validate` to `rivet config validate`
- Move `RIVET_CONCURRENT_UPLOADS` env var to CLI flag on appropriate commands (env var still works)
- Streamline `rivet init` experience
- Add `rivet token create public` command

### Fixed

- Fix `matchmaker.game_modes.*.docker.image_id` falling back to `matchmaker.docker.image_id`
- **Install script** Now installs non-prerelease GitHub releases

## [v0.3.0] - 2023-12-10

### Added

- **Install script (Unix)** Configure installation directory by passing `$BIN_DIR`
- **Install script (Unix)** Warning if `$BIN_DIR` is not in `$PATH`

### Changed

- Auto-generated & recommended config is now a `rivet.yaml` file
- Default version names are now generated as `YYYY.MM (X)` format (where `X` is an incrementing index)
- Merged `.rivet/cloud_token` and `.rivet/config.toml` in to unified internal `.rivet/config.yaml` config file
- **[BREAKING]** Removed support for file formats that are not YAML, TOML, or JSON in order to simplify maintaining forward compatibility
- **[BREAKING]** Throw error if both `.yaml` and `.yml` config exist

### Fixed

- **Install script (Unix)** Installing ARM64 `jq` binary on ARM-based Macs
- **Install script (Unix)** Automatically create `$BIN_DIR` if doesn't exist, specifically on macOS Sonoma which does not provide a `/usr/local/bin` by default

## [v0.2.0] - 2023-12-1

### Added

- Support for building OCI bundles
- Support for LZ4 compression of builds
- **[BREAKING]** Expose `RIVET_API_ENDPOINT` to `cdn.build_command` to help automate deploying to multiple clusters
- **[BREAKING]** Unset `RIVET_TOKEN` to `cdn.build_command` in order to ensure the cloud token isn't accidentally baked in a build
- `image build-push` command to automatically build & push an image
- `site build-push` command to automatially build and push a site
- E2E cross-platform tests in GitHub Actions

### Changed

- **[BREAKING]** Support new single-origin API endpoint (configured with `RIVET_API_ENDPOINT` environment variable or `--api-endpoint` flag)
- **[BREAKING]** Rename `RIVET_CLOUD_TOKEN` environment variable to `RIVET_TOKEN`
- **[BREAKING]** Rename `--cloud-token` flag to `--token`
- **[BREAKING]** Removed `RIVET_API_CLOUD_URL` in favor of `RIVET_API_ENDPOINT`
- **[BREAKING]** Updated custom games config schema
- **[BREAKING]** Removed domain map from turnstile configuration, replaced with `site_key` and `secret_key`
- Added telemetry beacon for fatal errors. Opt out with `--telemetry-disabled` or `TELEMETRY_DISABLED=1`
- Added internal config to store api endpoint and telemetry options
- Implemented multipart uploads for builds and sites, disable multipart uploads with `_RIVET_UPLOAD_DISABLE_MULTIPART`

## [v0.1.4] - 2023-12-9

### Added

- Darwin ARM release artifact

### Changed

- Update `cargo-dist` to 0.5.0

## [v0.1.3] - 2023-12-3

### Changed

- Replace Smithy-generated API library with OpenAPI-generated library in order to fix `invalid certificate timestamp: UnknownLog` error

## [v0.1.2] - 2023-08-26

### Changed

- Added custom games + lobby state + external verification

## [v0.1.1] - 2023-07-17

### Changed

- `rivet deploy` now gracefully falls back to the native build method if Docker Buildx is not installed

## [v0.1.0] - 2023-07-17

### Added

- Unreal helper in `rivet init`
- Installer for the Unreal Engine plugin with `rivet unreal install-plugin` or `rivet init --unreal`

### Changed

- Renamed `rivet.version.toml` to `rivet.toml`. All changes are backwards compatible.
- Renamed `rivet publish` command to `rivet deploy` since this is the more commonly used alias
- `rivet token create dev` now prints token in plain text

### Fixed

- Broken links to old docs
- Docker builder now catches missing builder errors correctly for older Docker versions

## [v0.0.51] - 2023-04-26

### Fixed

- Docker builder now catches missing builder errors correctly for older Docker versions

### Changed

- Remove `PORT`, `RIVET_LOBBY_TOKEN`, and `RIVET_PUBLIC_TOKEN` from generated .env file
- Document development token in .env

## [v0.0.50] - 2023-04-18

### Changed

- Description, homepage, and repository to Cargo.toml

### Fixed

- Incorrect package version

## [v0.0.49] - 2023-04-18

### Added

- Experimental build configuration flag `_RIVET_DOCKER_BUILD_METHOD` can be set to `buildx` or `native`

### Changed

- Default Docker build method is now Buildx, even if the native platform is x86
- Update dependency: `rivet-api`
- Upgrade dependency: `tokio 1.27`
- Removed unnecessary feature flags from `tokio`
