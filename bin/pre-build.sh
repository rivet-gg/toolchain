#!/bin/sh
set -euf -o pipefail

if [[ "$RUSTTARGET" == "x86_64-unknown-linux-musl" ]]; then
	echo "Instaling deps for x86_64-unknown-linux-musl"
	apk update
	apk add --no-cache pkgconfig openssl-dev gcc perl
fi

