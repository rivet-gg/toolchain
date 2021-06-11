#!/bin/bash
set -euo pipefail

BIN=./target/debug/rivetctl-cli

echo
echo
echo "=== BUILDING ==="
cargo build
$BIN --help

echo
echo
echo "=== AUTH TOKEN ==="
echo $CLOUD_TOKEN | $BIN --base-path $BASE_PATH auth token

echo
echo
echo "=== BUILD PUSH ==="
TEST_TAG=test-tag:latest
(
    cd tests/support/image
    docker build --tag $TEST_TAG .
)
$BIN --base-path $BASE_PATH build push $TEST_TAG
$BIN --base-path $BASE_PATH build push $TEST_TAG --name "Test Image"

echo
echo
echo "=== SITE PUSH ==="
$BIN --base-path $BASE_PATH site push tests/support/site/
$BIN --base-path $BASE_PATH site push tests/support/site/ --name "Test Site"
