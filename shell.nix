{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    clang
    libelf
    pkg-config
    zlib
  ];

  LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
}
