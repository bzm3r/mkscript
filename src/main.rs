
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
fn main() -> anyhow::Result<()> {
    let args = Interface::parse();
    let name = args.name;

    let sh = Shell::new()?;
    // TODO: if project already exists, ask if it should be deleted?
    cmd!(sh, "cargo init {name}").run()?;
    sh.change_dir(name);
    if args.cli {
        cmd!(sh, "cargo add clap --features derive").run()?;
    };
    cmd!(sh, "cargo add xshell anyhow").run()?;
    sh.remove_path("./src/main.rs")?;
    sh.write_file("./src/main.rs",
        r#"use xshell::{cmd, Shell};

        fn main() -> anyhow::Result<()> {
            let sh = Shell::new()?;
            cmd!(sh, "echo \"hello world!\"").run()?;
            Ok(())
        }
        "#)?;
    cmd!(sh, "rustfmt ./src/main.rs").run()?;
    cmd!(sh, "git init").run()?;
    Ok(())
}
