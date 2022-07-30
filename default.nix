{ rustPlatform, nix-gitignore }:

rustPlatform.buildRustPackage {
  name = "libosu";
  version = "0.0.28";

  src = nix-gitignore.gitignoreSource [ ./.gitignore ] ./.;

  cargoLock = {lockFile = ./Cargo.lock;};
}
