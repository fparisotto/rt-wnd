{
  description = "rt-wnd";

  inputs = {
    nixpkgs.url = github:NixOS/nixpkgs/nixos-unstable;
    flake-utils.url = github:numtide/flake-utils;

    rust-overlay.url = github:oxalica/rust-overlay;
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            rust-bin.stable.latest.default
            rust-analyzer
            pkg-config
            lld
          ];

          buildInputs = with pkgs; [
            libGL
            libxkbcommon
            mesa
            wayland
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr
            xorg.libxcb
          ];
          shellHook = ''
            export LD_LIBRARY_PATH="${pkgs.xorg.libX11}/lib"
          '';
        };

        packages.default = pkgs.rustPlatform.buildRustPackage rec {
          name = "rt-wnd";

          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };
        };
      }
    );
}
