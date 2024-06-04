{
  description = "test";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShell = pkgs.mkShell rec {
          nativeBuildInputs = with pkgs; [
            pkg-config
            rust-analyzer
          ];
          buildInputs = with pkgs; [
            openssl
            udev
            alsa-lib
            vulkan-loader
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr
            libxkbcommon
            wayland

            cargo-shuttle

            rustup
            mold
            llvmPackages_18.libcxxClang
          ];
          LD_LIBRARY_PATH = nixpkgs.lib.makeLibraryPath buildInputs;
        };
      }
    );
}
