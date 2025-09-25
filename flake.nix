{
  description = "rt-wnd";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
          config.allowUnfree = true;
        };
        rustToolchain = pkgs.rust-bin.stable.latest.default;
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            rustToolchain
            pkgs.rust-analyzer
            pkgs.cmake
            pkgs.clang
            pkgs.wayland
            pkgs.glfw
            pkgs.libpulseaudio
            pkgs.alsa-lib
            pkgs.sox
          ];
          LD_LIBRARY_PATH = with pkgs; lib.makeLibraryPath [
            libGL
            xorg.libXrandr
            xorg.libXinerama
            xorg.libXcursor
            xorg.libXi
            pkgs.libpulseaudio
            pkgs.alsa-lib
          ];
          LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
        };

        packages.default = pkgs.rustPlatform.buildRustPackage {
          name = "rt-wnd";
          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
        };
      }
    );
}
