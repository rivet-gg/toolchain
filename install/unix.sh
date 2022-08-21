#!/bin/sh
set -euf -o pipefail

VERSION=0.0.16
echo
echo "> Installing Rivet CLI @ $VERSION"

cd /tmp

if [ "$(uname)" == "Darwin" ]; then
	echo
	echo "> Detected macOS"

	echo
	URL="https://github.com/rivet-gg/cli/releases/download/${VERSION}/cli_${VERSION}_x86_64-apple-darwin.zip"
	echo "> Downloading $URL"
	curl -fsSL "$URL" -o rivet.zip

	echo
	echo "> Extracting rivet.zip"
	unzip rivet.zip

	echo
	echo "> Installing rivet"
	mv ./rivet /usr/local/bin/rivet
elif [ "$(expr substr $(uname -s) 1 5)" == "Linux" ]; then
	echo
	echo "> Detected Linux"

	echo
	URL="https://github.com/rivet-gg/cli/releases/download/${VERSION}/cli_${VERSION}_x86_64-unknown-linux-musl.tar.gz"
	echo "> Downloading $URL"
	curl -fsSL "$URL" -o rivet.tar.gz

	echo
	echo "> Extracting rivet.tar.gz"
	tar xzf rivet.tar.gz

	echo
	echo "> Installing rivet"
	mv ./rivet /usr/local/bin/rivet
else
	echo "Unable to determine platform" 1>&2
	exit 1
fi


echo
echo "> Checking installation"
rivet --help

