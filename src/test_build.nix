# run with `nix-build test_build.nix`
let pkgs = import <nixpkgs> { };

in pkgs.callPackage (import ./default.nix) { }
