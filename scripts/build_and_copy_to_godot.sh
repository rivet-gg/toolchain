#!/bin/sh

# macOS
cargo build
cp target/debug/rivet ../plugin-godot/addons/rivet/cli/rivet_x86_apple
