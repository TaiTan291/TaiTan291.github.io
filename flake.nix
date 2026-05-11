{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix/monthly";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    fenix,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [fenix.overlays.default];
      };

      toolchain = fenix.packages.${system}.complete.withComponents [
        "cargo"
        "clippy"
        "rust-src"
        "rustc"
        "rustfmt"
      ];

      rustWithWasm = fenix.packages.${system}.combine [
        toolchain
        fenix.packages.${system}.targets.wasm32-unknown-unknown.stable.rust-std
      ];
    in {
      devShells.default = pkgs.mkShell {
        buildInputs = [
          rustWithWasm
          pkgs.trunk
          pkgs.tailwindcss
          pkgs.rust-analyzer-nightly
          pkgs.alejandra
          
          # WASMビルドに必要なツール
          pkgs.wasm-bindgen-cli
          pkgs.binaryen
          pkgs.pkg-config
          pkgs.openssl
        ];

        shellHook = ''
          export RUST_SRC_PATH="${rustWithWasm}/lib/rustlib/src/rust/library"
        '';
      };

      formatter = pkgs.alejandra;
    })
    // {
      nixosConfigurations.nixos = nixpkgs.lib.nixosSystem {
        system = "x86_64-linux";
        modules = [
          ({pkgs, ...}: {
            nixpkgs.overlays = [fenix.overlays.default];
            environment.systemPackages = [
              pkgs.trunk
              pkgs.tailwindcss
              pkgs.alejandra
            ];
          })
        ];
      };
    };
}
