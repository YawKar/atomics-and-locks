{
  description = "Rust atomics and locks";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
          config.allowUnfree = true;
        };
        rust = {
          channel = "stable";
          version = "1.87.0";
          profile = "default";
        };
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            (rust-bin.${rust.channel}.${rust.version}.${rust.profile}.override {
              extensions = [ "rust-src" ];
            })
          ];
          shellHook = ''
            export PS1="(ral)$PS1"
          '';
        };
      }
    );
}
