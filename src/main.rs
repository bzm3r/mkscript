use clap::Parser;
use xshell::{cmd, Shell};

#[derive(Parser)]
#[command(author, version, about)]
struct Interface {
    /// New script's name.
    name: String,

    /// Whether the project requires CLI functionality (if so, `clap` will be
    /// added to its Cargo.toml).
    #[arg(long)]
    cli: bool,
}

const CLI: &str = r#"use clap::Parser;
#[derive(Parser)]
#[command(author, version, about)]
struct Interface {
    /// A string value
    name: String,
}"#;

const NAME_FROM_INTERFACE: &str = r#"let args = Interface::parse();
let name = args.name;"#;

const DEFAULT_NAME: &str = r#"let name = "world";"#;

const MAIN_RS: fn(bool) -> String = |cli: bool| {
    format!(
        r#"{}use xshell::{{cmd, Shell}};
fn main() -> anyhow::Result<()> {{
    {}
    let sh = Shell::new()?;
    cmd!(sh, "echo \"hello {{name}}!\"").run()?;
    Ok(())
}}
"#,
        if cli { CLI } else { "" },
        if cli {
            NAME_FROM_INTERFACE
        } else {
            DEFAULT_NAME
        }
    )
};

// Double brackets in the body of this functions's format string in order to
// escape them and print them as usual.
const DEFAULT_NIX: fn(&str) -> String = |script_name: &str| {
    format!(
        r#"{{ lib, rustPlatform }}:
rustPlatform.buildRustPackage rec {{
  pname = "{script_name}";
  version = "0.1.0";

  src = ./.;

  cargoLock = {{ lockFile = ./Cargo.lock; }};

  buildType = "release";

  meta = with lib; {{
    description = "Helper for creating a new Rust scripting project";
    homepage = "https://github.com/bzm3r/{script_name}";
    license = with licenses; [ asl20 mit ];
    maintainers = with maintainers; [ ];
    mainProgram = "{script_name}";
  }};
}}
"#
    )
};

const TEST_BUILD_NIX: &str = r#"# run with `nix-build test_build.nix`
let
  pkgs = import <nixpkgs> {};
in
pkgs.callPackage (import ./default.nix) {}
"#;

fn main() -> anyhow::Result<()> {
    let args = Interface::parse();
    let name = args.name;

    let sh = Shell::new()?;
    // TODO: if project already exists, ask if it should be deleted?
    cmd!(sh, "cargo init {name}").run()?;
    sh.change_dir(&name);
    if args.cli {
        cmd!(sh, "cargo add clap --features derive").run()?;
    };
    cmd!(sh, "cargo add xshell anyhow").run()?;
    sh.remove_path("./src/main.rs")?;
    sh.write_file("./src/main.rs", MAIN_RS(args.cli))?;
    sh.write_file("./default.nix", DEFAULT_NIX(&name))?;
    sh.write_file("./test_build.nix", TEST_BUILD_NIX)?;
    cmd!(sh, "rustfmt ./src/main.rs").run()?;

    let interact_with_reuse = format!("Apache-2.0\nMIT\n\n{name}\nhttps://github.com/bzm3r/{name}\nBrian Merchant\nbzm3r@proton.me\n");
    cmd!(sh, "reuse init").stdin(&interact_with_reuse).run()?;

    cmd!(sh, "git init").run()?;
    let original_contents = sh.read_file(".gitignore")?;
    sh.write_file(".gitignore", format!("{original_contents}/result\n"))?;
    cmd!(sh, "git add .").run()?;
    cmd!(sh, "git commit -m \"init\"").run()?;

    Ok(())
}
