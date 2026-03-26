{
  description = "rt-wnd";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    flake-utils = {
      url = "github:numtide/flake-utils";
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
        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustToolchain;
          rustc = rustToolchain;
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            rustToolchain
            pkgs.rust-analyzer
            pkgs.cmake
            pkgs.clang
            pkgs.mesa
            pkgs.wayland
            pkgs.glfw
            pkgs.libxkbcommon
            pkgs.libdecor
            pkgs.libpulseaudio
            pkgs.alsa-lib
            pkgs.sox
          ];
          LD_LIBRARY_PATH = with pkgs; lib.makeLibraryPath [
            libGL
            mesa
            wayland
            libxkbcommon
            libdecor
            xorg.libXrandr
            xorg.libXinerama
            xorg.libXcursor
            xorg.libXi
            pkgs.libpulseaudio
            pkgs.alsa-lib
          ];
          LIBGL_DRIVERS_PATH = "${pkgs.mesa}/lib/dri";
          __EGL_VENDOR_LIBRARY_DIRS = "${pkgs.mesa}/share/glvnd/egl_vendor.d";
          LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
        };

        packages.default = rustPlatform.buildRustPackage {
          name = "rt-wnd";
          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
          nativeBuildInputs = [
            pkgs.cmake
            pkgs.pkg-config
            pkgs.clang
          ];
          buildInputs = [
            pkgs.mesa
            pkgs.wayland
            pkgs.glfw
            pkgs.libxkbcommon
            pkgs.libdecor
            pkgs.libpulseaudio
            pkgs.alsa-lib
            pkgs.libclang
          ];
          LIBGL_DRIVERS_PATH = "${pkgs.mesa}/lib/dri";
          __EGL_VENDOR_LIBRARY_DIRS = "${pkgs.mesa}/share/glvnd/egl_vendor.d";
          LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
        };
      }
    );
}
