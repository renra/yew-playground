{ nixpkgs ? import <nixpkgs> {  } }:

let
  pkgs = [
    nixpkgs.rustup
  ];

in
  nixpkgs.stdenv.mkDerivation {
    name = "yew-playground";
    buildInputs = pkgs;
  }
