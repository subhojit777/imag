{ pkgs ? (import <nixpkgs> {}) }:

let
  env = with pkgs.rustStable; [
    rustc
    cargo
  ];

  dependencies = with pkgs; [
    openssl
    zlib
    cmake
    which
  ];
in

pkgs.stdenv.mkDerivation rec {
    name = "imag";
    src = ./.;
    version = "0.0.0";

    buildInputs = env ++ dependencies;

}

