{ lib, rustPlatform }:
rustPlatform.buildRustPackage {
  pname = "mkscript";
  version = "0.1.0";

  src = ./.;

  cargoLock = { lockFile = ./Cargo.lock; };
  cargoBuildFlags = [ "--bin" "mkscript" ];

  buildType = "release";

  meta = with lib; {
    description = "Helper for creating a new Rust scripting project";
    homepage = "https://github.com/bzm3r/mkscript";
    license = with licenses; [ asl20 mit ];
    mainProgram = "mkscript";
  };
}
