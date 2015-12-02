{ pkgs ? (import <nixpkgs> {}) }:

let
  env = with pkgs.rustUnstable; [
    rustc
    cargo
    pkgs.llvmPackages.lldb
  ];
in

pkgs.stdenv.mkDerivation rec {
    name = "imag";
    src = ./.;
    version = "0.0.0";

    buildInputs = [ env ];

}

