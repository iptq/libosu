{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    utils.url = "github:numtide/flake-utils";
    fenix.url = "github:nix-community/fenix";
  };

  outputs = { self, nixpkgs, fenix, utils }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ fenix.overlay ];
        };

        myPkgs = rec {
          libosu = (import ./Cargo.nix { inherit pkgs; }).rootCrate.build;
        };
      in rec {
        packages = utils.lib.flattenTree myPkgs;
        defaultPackage = packages.libosu;

        devShell = pkgs.mkShell {
          CARGO_UNSTABLE_SPARSE_REGISTRY = "true";
          inputsFrom = with packages; [ libosu ];
          packages = with pkgs;
            with pkgs.fenix.minimal; [
              cargo
              cargo-all-features
              cargo-edit
              cargo-udeps
              cargo-watch
              crate2nix
              rustfmt
            ];
        };
      });
}
