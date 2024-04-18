{
  description = "test";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs =
    { self, nixpkgs, ... }@inputs:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      devShells.${system}.default = pkgs.mkShell rec {
        nativeBuildInputs = with pkgs; [
          nodejs
          pkg-config
          rust-analyzer
        ];
        buildInputs = with pkgs; [
          openssl
          udev alsa-lib vulkan-loader
          xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr
          libxkbcommon wayland
          
          rustup
          mold
          llvmPackages_18.libcxxClang
        ];
        LD_LIBRARY_PATH = nixpkgs.lib.makeLibraryPath buildInputs;
      };
    };
}
