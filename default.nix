{ pkgs ? import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/nixpkgs-unstable.tar.gz") {} }:

pkgs.stdenv.mkDerivation {
  name = "valentine";

  buildInputs = [
    pkgs.rustc
    pkgs.cargo
  ];

  src = ./.;
}
