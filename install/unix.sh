#!/bin/sh
set -euf -o pipefail

VERSION=0.0.16

cd /tmp
curl --proto '=https' --tlsv1.2 -sSfL "https://github.com/rivet-gg/cli/releases/download/${VERSION}/cli_${VERSION}_x86_64-unknown-linux-musl.tar.gz" -o rivet.tar.gz
tar xzf rivet.tar.gz
mv ./rivet /usr/local/bin/rivet

