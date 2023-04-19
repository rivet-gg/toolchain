#!/bin/sh
# shellcheck enable=add-default-case
# shellcheck enable=avoid-nullary-conditions
# shellcheck enable=check-unassigned-uppercase
# shellcheck enable=deprecate-which
# shellcheck enable=quote-safe-variables
# shellcheck enable=require-variable-braces
set -eu

rm -rf /tmp/rivet_cli_install
mkdir /tmp/rivet_cli_install
cd /tmp/rivet_cli_install

UNAME="$(uname -s)"

# Find asset suffix
if [ "$(printf '%s' "$UNAME" | cut -c 1-6)" = "Darwin" ]; then
	echo
	echo "> Detected macOS"

	echo
	echo "> Installing jq"
	curl -fsSL "https://github.com/stedolan/jq/releases/download/jq-1.6/jq-osx-amd64" -o ./jq
	chmod +x ./jq

	CLI_ASSET_SUFFIX="-x86_64-apple-darwin.tar.xz"
elif [ "$(printf '%s' "$UNAME" | cut -c 1-5)" = "Linux" ]; then
	echo
	echo "> Detected Linux ($(getconf LONG_BIT) bit)"

	echo
	echo "> Installing jq"
	curl -fsSL "https://github.com/stedolan/jq/releases/download/jq-1.6/jq-linux$(getconf LONG_BIT)" -o ./jq
	chmod +x ./jq

	CLI_ASSET_SUFFIX="-x86_64-unknown-linux-gnu.tar.xz"
else
	echo "Unable to determine platform" 1>&2
	exit 1
fi

# Find CLI version
set +u
if [ -z "$RIVET_CLI_VERSION" ]; then
	echo
	echo "> Fetching latest release version"
	RIVET_CLI_VERSION="$( \
		curl -fsSL https://api.github.com/repos/rivet-gg/cli/releases \
		| ./jq -re \
			--arg cli_asset_suffix "$CLI_ASSET_SUFFIX" \
			'[.[] | select(.assets[] | select(.name | endswith($cli_asset_suffix)))] | first | .name' \
	)"
fi
set -u

echo
echo "> Installing Rivet CLI @ $RIVET_CLI_VERSION"


if [ "$(printf '%s' "$UNAME" | cut -c 1-6)" = "Darwin" ]; then
	echo
	ASSET_NAME="rivet-cli-${RIVET_CLI_VERSION}${CLI_ASSET_SUFFIX}"
	URL="https://github.com/rivet-gg/cli/releases/download/${RIVET_CLI_VERSION}/${ASSET_NAME}"
	echo "> Downloading $URL"
	curl -fsSL "$URL" -o rivet_cli.tar.xz

	echo
	echo "> Extracting rivet.zip"
	tar xJf rivet_cli.tar.xz

	echo
	echo "> Installing rivet"
	sudo mv "./rivet-cli-${RIVET_CLI_VERSION}-x86_64-apple-darwin/rivet-cli" "/usr/local/bin/rivet"
elif [ "$(printf '%s' "$UNAME" | cut -c 1-5)" = "Linux" ]; then
	echo
	ASSET_NAME="rivet-${RIVET_CLI_VERSION}${CLI_ASSET_SUFFIX}"
	URL="https://github.com/rivet-gg/cli/releases/download/${RIVET_CLI_VERSION}/${ASSET_NAME}"
	echo "> Downloading $URL"
	curl -fsSL "$URL" -o rivet_cli.tar.xz

	echo
	echo "> Extracting rivet.tar.gz"
	tar xJf rivet_cli.tar.xz

	echo
	echo "> Installing rivet"
	if command -v sudo; then
		sudo mv "./rivet-cli-${RIVET_CLI_VERSION}-x86_64-unknown-linux-gnu/rivet-cli" "/usr/local/bin/rivet"
	else
		mv "./rivet-cli-${RIVET_CLI_VERSION}-x86_64-unknown-linux-gnu/rivet-cli" "/usr/local/bin/rivet"
	fi
else
	exit 1
fi

echo
echo "> Checking installation"
rivet --version

echo
echo "Rivet was installed successfully."
echo "Run 'rivet --help' to get started."
