{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = {
    self,
    nixpkgs,
    flake-utils,
    treefmt-nix,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};
        # treefmtの設定を評価
        treefmtStack = treefmt-nix.lib.evalModule pkgs {
          projectRootFile = "flake.nix";
          programs = {
            alejandra.enable = true; # Nix用
            rustfmt.enable = true; # Rust用
            prettier.enable = true; # Unocss用
          };
        };
      in {
        formatter = treefmtStack.config.build.wrapper;

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = [
            # Rust関連
            pkgs.rustc
            pkgs.cargo
            pkgs.trunk

            # Formatter郡
            treefmtStack.config.build.wrapper
          ];
        };
      }
    );
}
