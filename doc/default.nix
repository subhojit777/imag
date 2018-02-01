{ pkgs ? (import <nixpkgs> {}) }:

let
  env = with pkgs.haskellPackages; [
    pandoc
    pandoc-crossref

    (pkgs.texlive.combine {
      inherit (pkgs.texlive)
        scheme-basic

        amsfonts
        amsmath
        lm
        ifxetex
        ifluatex
        eurosym
        listings
        fancyvrb
        # longtable
        booktabs

        # no graphics...
        # graphicx
        # grffile

        hyperref
        ulem
        geometry
        setspace
        babel

        subfig
        caption

        # optionals
        upquote
        microtype
        csquotes

        # We have no citation support
        # natbib
        # biblatex
        # bibtex
        # biber

        # some more, not listed in the pandoc docs
        mathtools
        enumitem
      ;
    })

    pkgs.lmodern

    pkgs.which
  ];
in

pkgs.stdenv.mkDerivation rec {
    name = "imag-doc";
    src = /var/empty;
    version = "0.0.0";

    buildInputs = [ env ];

}

