{ pkgs ? (import <nixpkgs> {}) }:

let
  env = with pkgs.rustStable; [
    rustc
    cargo
  ];

  dependencies = with pkgs; [
    ruby
    bundler
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

