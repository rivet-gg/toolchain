let
	moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/80627b282705101e7b38e19ca6e8df105031b072.tar.gz);
	pkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
in
	pkgs.mkShell {
		name = "rivet-cli";
		buildInputs = with pkgs; [
			cacert

			pkgs.latest.rustChannels.stable.rust
			pkg-config
			openssl
			libiconv

			shellcheck
		] ++ (
			pkgs.lib.optionals stdenv.isDarwin [
				darwin.apple_sdk.frameworks.Security
			]
		);
	}

