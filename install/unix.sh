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
ARCH="$(uname -m)"

# Find asset suffix
if [ "$(printf '%s' "$UNAME" | cut -c 1-6)" = "Darwin" ]; then
	echo
	echo "> Detected macOS"

	echo
	echo "> Installing jq"
	if [ "$ARCH" = "x86_64" ]; then
		url="https://github.com/jqlang/jq/releases/download/jq-1.7/jq-macos-amd64"
	elif [ "$ARCH" = "arm64" ]; then
		url="https://github.com/jqlang/jq/releases/download/jq-1.7/jq-macos-arm64"
	fi
	curl -fsSL "$url" -o ./jq
	chmod +x ./jq


	CLI_ASSET_SUFFIX="-x86_64-apple-darwin.tar.xz"
elif [ "$(printf '%s' "$UNAME" | cut -c 1-5)" = "Linux" ]; then
	echo
	echo "> Detected Linux ($(getconf LONG_BIT) bit)"

	echo
	echo "> Installing jq"
	curl -fsSL "https://github.com/stedolan/jq/releases/download/jq-1.7/jq-linux$(getconf LONG_BIT)" -o ./jq
	chmod +x ./jq

	CLI_ASSET_SUFFIX="-x86_64-unknown-linux-gnu.tar.xz"
else
	echo "Unable to determine platform" 1>&2
	exit 1
fi

# Determine install location
set +u
if [ -z "$BIN_DIR" ]; then
	BIN_DIR="/usr/local/bin"
fi
set -u
INSTALL_PATH="$BIN_DIR/rivet"

if [ ! -d "$BIN_DIR" ]; then

	if command -v sudo; then
        echo
        echo "> Creating directory $BIN_DIR (requires sudo)"
        sudo mkdir -p $BIN_DIR
	else
        echo
        echo "> Creating directory $BIN_DIR"
        mkdir -p $BIN_DIR
	fi
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
			'[.[] | select(.assets[] | select(.name | endswith($cli_asset_suffix)))] | first | .tag_name' \
	)"
fi
set -u

echo
echo "> Installing Rivet CLI $RIVET_CLI_VERSION"

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
    echo "> Installing rivet to $INSTALL_PATH (requires sudo)"
	sudo mv "./rivet-cli-${RIVET_CLI_VERSION}-x86_64-apple-darwin/rivet-cli" "$INSTALL_PATH"
elif [ "$(printf '%s' "$UNAME" | cut -c 1-5)" = "Linux" ]; then
	echo
	ASSET_NAME="rivet-cli-${RIVET_CLI_VERSION}${CLI_ASSET_SUFFIX}"
	URL="https://github.com/rivet-gg/cli/releases/download/${RIVET_CLI_VERSION}/${ASSET_NAME}"
	echo "> Downloading $URL"
	curl -fsSL "$URL" -o rivet_cli.tar.xz

	echo
	echo "> Extracting rivet.tar.gz"
	tar xJf rivet_cli.tar.xz

	if command -v sudo; then
        echo
        echo "> Installing rivet to $INSTALL_PATH (requires sudo)"
		sudo mv "./rivet-cli-${RIVET_CLI_VERSION}-x86_64-unknown-linux-gnu/rivet-cli" "$INSTALL_PATH"
	else
        echo
        echo "> Installing rivet to $INSTALL_PATH"
		mv "./rivet-cli-${RIVET_CLI_VERSION}-x86_64-unknown-linux-gnu/rivet-cli" "$INSTALL_PATH"
	fi
else
	exit 1
fi

case ":$PATH:" in
    *:$BIN_DIR:*) ;;
    *) echo "WARNING: $BIN_DIR is not in \$PATH" ;;
esac

echo
echo "> Checking installation"
"$BIN_DIR/rivet" --version

echo
echo "Rivet was installed successfully."
echo "Run 'rivet --help' to get started."
