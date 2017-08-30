{ pkgs ? (import <nixpkgs> {}) }:

let
  env = with pkgs.latest.rustChannels.stable; [
    rust
    cargo
  ];

  dependencies = with pkgs; [
    cmake
    curl
    gcc
    libpsl
    openssl
    pkgconfig
    which
    zlib
  ];
in

pkgs.stdenv.mkDerivation rec {
    name = "imag";
    src = ./.;
    version = "0.0.0";

    buildInputs = env ++ dependencies;

}

