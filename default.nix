let pkgs = import <nixpkgs> {};
in pkgs.callPackage ({ lib, rustPlatform }:

  rustPlatform.buildRustPackage rec {
    pname = "create_script";
    version = "unstable-2023-12-12";

    src = ./.;

    cargoLock = { lockFile = ./Cargo.lock; };

    meta = with lib; {
      description = "Helper for creating a new Rust scripting project";
      homepage = "https://github.com/bzm3r/create_script";
      license = with licenses; [ asl20 mit ];
      maintainers = with maintainers; [ ];
      mainProgram = "create_script";
    };
  }) {}
