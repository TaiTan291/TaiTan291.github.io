{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
		rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = {
    self,
    nixpkgs,
    flake-utils,
    treefmt-nix,
		rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
				＃rust-overlays
      	pkgs = import nixpkgs {
      		inherit system;
      		overlays = [ (import rust-overlay) ];
    		};
    		rustToolchain = pkgs.rust-bin.stable.latest.default.override {
      		extensions = [ "rust-src" "rust-analyzer" "clippy" "rustfmt" ];
    		};

        # treefmt
        treefmtStack = treefmt-nix.lib.evalModule pkgs {
          projectRootFile = "flake.nix";
          programs = {
            alejandra.enable = true; # Nix用
            rustfmt = {
							enable = true; # Rust用
							package = rustToolchain;
						};
            prettier.enable = true; # HTML/CSS/JS用
          };
        };
      in {
        formatter = treefmtStack.config.build.wrapper;

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = [
            # Rust関連
            rustToolchain # Rustツール群
            pkgs.trunk

            # CSS
            pkgs.tailwindcss

            # Formatter郡
            treefmtStack.config.build.wrapper
          ];
        };
      }
    );
}
