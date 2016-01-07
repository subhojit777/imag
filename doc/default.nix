{ pkgs ? (import <nixpkgs> {}) }:

let
  env = with pkgs.haskellPackages; [
    pandoc

    (pkgs.texlive.combine {
      inherit (pkgs.texlive)
        scheme-small
        algorithms
        cm-super
        collection-basic
        collection-fontsextra
        collection-fontutils
        collection-langenglish
        collection-latex
        collection-latexextra
        collection-latexrecommended
        collection-mathextra
        collection-pictures
        collection-plainextra
        collection-science
      ;
    })

    pkgs.lmodern
  ];
in

pkgs.stdenv.mkDerivation rec {
    name = "imag-doc";
    src = ./.;
    version = "0.0.0";

    buildInputs = [ env ];

}

