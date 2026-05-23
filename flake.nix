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

      # 1. ツールチェーンを一貫性のある形で定義 (latest または stable に統一)
      # 2. ターゲットの rust-std も含めて一気に合成する
      rustWithWasm = fenix.packages.${system}.combine [
        (fenix.packages.${system}.latest.withComponents [
          "cargo"
          "clippy"
          "rust-src"
          "rustc"
          "rustfmt"
          "rust-std" # ホスト用
        ])
        fenix.packages.${system}.targets.wasm32-unknown-unknown.latest.rust-std # Wasm用
      ];
    in {
      devShells.default = pkgs.mkShell {
        # 実行時に必要なもの
        buildInputs = [
          rustWithWasm
          pkgs.trunk
          pkgs.tailwindcss
          pkgs.alejandra
        ];

        # コンパイル（ビルド時）に必要なもの
        nativeBuildInputs = with pkgs; [
          pkg-config
          wasm-bindgen-cli
          binaryen
          openssl
				];

        shellHook = ''
          export RUST_SRC_PATH="${rustWithWasm}/lib/rustlib/src/rust/library"
        '';
      };

      formatter = pkgs.alejandra;
    });
}
