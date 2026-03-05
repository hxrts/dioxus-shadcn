{
  description = "Lumen Blocks - Accessible, styled components for Dioxus";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # Rust toolchain - using nightly for edition 2024 support
        rustToolchain = pkgs.rust-bin.nightly.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
          targets = [ "wasm32-unknown-unknown" ];
        };

        # Darwin-specific inputs
        darwinInputs = pkgs.lib.optionals pkgs.stdenv.isDarwin [
          pkgs.libiconv
        ];

        # Common build inputs for development
        buildInputs = with pkgs; [
          # Rust toolchain
          rustToolchain

          # Dioxus CLI
          dioxus-cli

          # Build tools
          just
          pkg-config

          # For wasm builds
          wasm-bindgen-cli
          binaryen  # wasm-opt

          # SSL/TLS support
          openssl
        ] ++ darwinInputs;

        # Native build inputs
        nativeBuildInputs = with pkgs; [
          pkg-config
        ];

      in
      {
        # Development shell
        devShells.default = pkgs.mkShell {
          inherit buildInputs nativeBuildInputs;

          shellHook = ''
            echo "Lumen Blocks development environment"
            echo "Rust: $(rustc --version)"
            echo "Dioxus CLI: $(dx --version)"
            echo ""
            echo "Commands:"
            echo "  just dev-docsite    - Start the docsite dev server"
            echo "  just build-docsite  - Build the docsite for production"
            echo "  cargo build         - Build the library"
            echo "  cargo test          - Run tests"
          '';

          # Environment variables
          RUST_BACKTRACE = "1";
          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
        };

        # Package for building the docsite
        packages.docsite = pkgs.stdenv.mkDerivation {
          pname = "lumen-blocks-docsite";
          version = "0.3.0";

          src = ./.;

          nativeBuildInputs = nativeBuildInputs ++ [ rustToolchain pkgs.dioxus-cli ];

          buildInputs = buildInputs;

          buildPhase = ''
            export HOME=$(mktemp -d)
            export CARGO_HOME=$HOME/.cargo
            dx bundle -p docsite --platform web --features analytics --release
          '';

          installPhase = ''
            mkdir -p $out
            cp -r target/dx/docsite/release/web/public/* $out/
            if [ -f docsite/assets/_redirects ]; then
              cp docsite/assets/_redirects $out/
            fi
          '';
        };

        # Library package (for use as a dependency)
        packages.lumen-blocks = pkgs.rustPlatform.buildRustPackage {
          pname = "lumen-blocks";
          version = "0.3.0";

          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
            allowBuiltinFetchGit = true;
          };

          nativeBuildInputs = nativeBuildInputs;
          buildInputs = with pkgs; [
            openssl
          ] ++ darwinInputs;

          buildPhase = ''
            cargo build --package lumen-blocks --release
          '';

          installPhase = ''
            mkdir -p $out/lib
            if [ -f target/release/liblumen_blocks.rlib ]; then
              cp target/release/liblumen_blocks.rlib $out/lib/
            fi
          '';

          doCheck = false;  # Tests require browser environment
        };

        packages.default = self.packages.${system}.docsite;
      }
    );
}
