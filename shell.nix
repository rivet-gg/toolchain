let
    # Include most recent Rust builds
    moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
    pkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
in
    pkgs.mkShell {
        name = "rivet-cli";
        buildInputs = with pkgs; [
            cacert

            (pkgs.latest.rustChannels.stable.rust.override {
                targets = [ "wasm32-unknown-unknown" ];
            })
            pkg-config
            openssl
        ];
    }

