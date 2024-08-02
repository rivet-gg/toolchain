let
	moz_overlay = import (builtins.fetchTarball "https://github.com/mozilla/nixpkgs-mozilla/archive/6eabade97bc28d707a8b9d82ad13ef143836736e.tar.gz");
	pkgs = import (fetchTarball {
		url = "https://github.com/NixOS/nixpkgs/archive/refs/tags/23.05.tar.gz";
	}) { overlays = [ moz_overlay ]; };
in
	pkgs.mkShell {
		name = "rivet-cli";
		buildInputs = with pkgs; [
			cacert

			# pkgs.latest.rustChannels.stable.rust
      (pkgs.latest.rustChannels.stable.rust.override {
       extensions = [ "rust-src" "rust-std" ];
       targets = [
         "x86_64-unknown-linux-gnu"
         "x86_64-pc-windows-msvc"
         "x86_64-apple-darwin"
         "aarch64-apple-darwin"
       ];
     })
			pkg-config
			perl

			# Libraries
			openssl
			libiconv
			zlib

			shellcheck
		] ++ (
			pkgs.lib.optionals stdenv.isDarwin [
				darwin.apple_sdk.frameworks.Security
				darwin.apple_sdk.frameworks.CoreServices
				darwin.apple_sdk.frameworks.SystemConfiguration
			]
		);
	}

