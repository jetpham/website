{
  description = "CTF Jet development environment (Bun)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        bun = pkgs.bun;

        # Prisma engines for NixOS
        prismaEngines = pkgs.prisma-engines;

        devTools = with pkgs; [
          git
          postgresql
          curl
          wget
          typescript-language-server
          pkg-config
          wasm-pack
          binaryen
          (rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
            extensions = [ "rust-src" ];
            targets = [ "wasm32-unknown-unknown" ];
          }))
        ];

      in {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            bun
            prismaEngines
          ] ++ devTools;

          NIXPKGS_ALLOW_UNFREE = "1";

          PRISMA_QUERY_ENGINE_BINARY = "${prismaEngines}/bin/query-engine";
          PRISMA_SCHEMA_ENGINE_BINARY = "${prismaEngines}/bin/schema-engine";
          PRISMA_INTROSPECTION_ENGINE_BINARY = "${prismaEngines}/bin/introspection-engine";
          PRISMA_ENGINES_CHECKSUM_IGNORE_MISSING = "1";
        };

        packages = {
          inherit bun prismaEngines;

          default = pkgs.symlinkJoin {
            name = "ctfjet-dev-bun";
            paths = [ bun prismaEngines ];
          };
        };
      }
    );
}
