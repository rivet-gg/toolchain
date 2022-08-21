#!/bin/sh
set -euf -o pipefail

cd /tmp
curl TODO -o rivet.tar.gz
tar xf rivet.tar.gz
mv ./rivet /usr/local/bin/rivet

