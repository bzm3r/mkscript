{ lib, rustPlatform }:
rustPlatform.buildRustPackage {
  pname = "TEMPLATE_PLACEHOLDER";
  version = "0.1.0";

  src = ./.;

  cargoLock = { lockFile = ./Cargo.lock; };

  buildType = "release";

  meta = with lib; {
    description = "Helper for creating a new Rust scripting project";
    homepage = "https://github.com/bzm3r/TEMPLATE_PLACEHOLDER";
    license = with licenses; [ asl20 mit ];
    mainProgram = "TEMPLATE_PLACEHOLDER";
  };
}
