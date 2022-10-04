#!/bin/sh
set -eu

# Find asset suffix
if [ "$(uname)" = "Darwin" ]; then
	echo
	echo "> Detected macOS"

	CLI_ASSET_SUFFIX="_x86_64-apple-darwin.zip"
elif [ "$(expr substr "$(uname -s)" 1 5)" = "Linux" ]; then
	echo
	echo "> Detected Linux"

	CLI_ASSET_SUFFIX="_x86_64-unknown-linux-musl.tar.gz"
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
		curl -fsSL https://api.github.com/repos/rivet-gg/cli/releases
		| jq -re \
			--arg cli_asset_suffix "$CLI_ASSET_SUFFIX" \
			'[.[] | select(.assets[] | select(.name | endswith($cli_asset_suffix)))] | first | .name' \
	)"
fi
set -u

echo
echo "> Installing Rivet CLI @ $RIVET_CLI_VERSION"

cd /tmp

if [ "$(uname)" = "Darwin" ]; then
	echo
	URL="https://github.com/rivet-gg/cli/releases/download/${RIVET_CLI_VERSION}/cli_${RIVET_CLI_VERSION}_x86_64-apple-darwin.zip"
	echo "> Downloading $URL"
	curl -fsSL "$URL" -o rivet.zip

	echo
	echo "> Extracting rivet.zip"
	unzip rivet.zip

	echo
	echo "> Installing rivet"
	sudo mv ./rivet /usr/local/bin/rivet
elif [ "$(expr substr "$(uname -s)" 1 5)" = "Linux" ]; then
	echo
	URL="https://github.com/rivet-gg/cli/releases/download/${RIVET_CLI_VERSION}/cli_${RIVET_CLI_VERSION}_x86_64-unknown-linux-musl.tar.gz"
	echo "> Downloading $URL"
	curl -fsSL "$URL" -o rivet.tar.gz

	echo
	echo "> Extracting rivet.tar.gz"
	tar xzf rivet.tar.gz

	echo
	echo "> Installing rivet"
	if command -v sudo; then
		sudo mv ./rivet /usr/local/bin/rivet
	else
		mv ./rivet /usr/local/bin/rivet
	fi
else
	exit 1
fi


echo
echo "> Checking installation"
rivet --help

