{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
  }:
    flake-utils.lib.eachDefaultSystem
    (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
        with pkgs; {
          formatter = alejandra;

          devShells.default = mkShell {
            RUST_SRC_PATH = "${rust.packages.stable.rustPlatform.rustLibSrc}";
            PROTOC = "${protobuf}/bin/protoc";
            PROTOC_INCLUDE = "${protobuf}/include";

            packages = [
              bun
              just
              protobuf
            ];

            nativeBuildInputs = [
              pkg-config
            ];

            buildInputs = [
              (rust-bin.stable.latest.default.override {
                extensions = ["rust-src"];
              })
            ];
          };
        }
    );
}
