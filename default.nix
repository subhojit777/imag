{ stdenv
, pkgs ? (import <nixpkgs> {})
}:

let
  env = with pkgs.rustPlatform; [
    rustc
    cargo
  ];
in

stdenv.mkDerivation rec {
    name = "unfug.org";
    src = ./.;
    version = "0.0.0";

    buildInputs = [ env ];

}

