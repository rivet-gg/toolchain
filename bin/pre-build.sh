#!/bin/sh
set -euf -o pipefail

apk update
apk add --no-cache pkgconfig openssl-dev gcc musl-dev

