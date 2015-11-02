{ pkgs ? (import <nixpkgs> {}) }:

let
  env = with pkgs.rustPlatform; [
    rustc
    cargo
  ];
in

pkgs.stdenv.mkDerivation rec {
    name = "imag";
    src = ./.;
    version = "0.0.0";

    buildInputs = [ env ];

}

