#!/bin/sh
set -euf -o pipefail

echo "Installing pkgconfig & openssl-dev"
apk update
apk add --no-cache pkgconfig openssl-dev

