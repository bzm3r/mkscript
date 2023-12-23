use std::{fs::OpenOptions, io::Write};

use bpaf::{self, Bpaf, Parser};
use xshell::{cmd, Shell};

/// Initialize a template Rust project for a shell script or CLI tool.
#[derive(Bpaf, Debug, Clone)]
struct MkScript {
    /// Whether the project requires CLI functionality (if so, `clap` will be
    /// added to its Cargo.toml).
    #[bpaf(short, long)]
    cli: bool,

    /// Whether the project should be associated with a Github repo. If a repo
    /// name is provided, the repo will be created under that name. Otherwise,
    /// the script's name will be used as the repo's name.
    #[bpaf(short, long)]
    gh: bool,

    /// New script's name.
    #[bpaf(positional("SCRIPT_NAME"))]
    script_name: String,
}

struct Templates {
    cli_main_rs: &'static str,
    shell_main_rs: &'static str,
    default_nix: fn(&str) -> String,
    test_build_nix: &'static str,
    rustfmt_toml: &'static str,
}

const TEMPLATES: Templates = Templates {
    cli_main_rs: include_str!("template_cli.rs"),
    shell_main_rs: include_str!("template_shell.rs"),
    default_nix: |script_name| {
        include_str!("../default.nix")
            .replace("TEMPLATE_PLACEHOLDER", script_name)
    },
    test_build_nix: include_str!("../test_build.nix"),
    rustfmt_toml: include_str!("rustfmt.toml"),
};

fn main() -> anyhow::Result<()> {
    let opts = mk_script().run();
    let script_name = opts.script_name;

    let sh = Shell::new()?;
    cmd!(sh, "cargo init {script_name}").run()?;
    sh.change_dir(&script_name);
    if opts.cli {
        cmd!(
            sh,
            "cargo add bpaf --features autocomplete,docgen,bright-color,derive"
        )
        .run()?;
    };
    cmd!(sh, "cargo add xshell anyhow").run()?;
    sh.remove_path("./src/main.rs")?;
    sh.write_file(
        "./src/main.rs",
        if opts.cli {
            TEMPLATES.cli_main_rs
        } else {
            TEMPLATES.shell_main_rs
        },
    )?;
    sh.write_file("./default.nix", (TEMPLATES.default_nix)(&script_name))?;
    sh.write_file("./test_build.nix", TEMPLATES.test_build_nix)?;
    sh.write_file("./rustfmt.toml", TEMPLATES.rustfmt_toml)?;
    cmd!(sh, "rustfmt ./src/main.rs").run()?;

    let interact_with_reuse = format!("Apache-2.0\nMIT\n\n{script_name}\nhttps://github.com/bzm3r/{script_name}\nBrian Merchant\nbzm3r@proton.me\n");

    cmd!(sh, "reuse init")
        .stdin(&interact_with_reuse)
        .run()
        .unwrap_or_else(|err| println!("{err}"));

    cmd!(sh, "git init").run()?;

    match OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(sh.current_dir().join(".gitignore"))
    {
        Ok(mut f) => {
            f.write_all("/result\n".as_bytes()).unwrap_or_else(|err| {
                println!("Could not append to .gitignore: {err}")
            });
        }
        Err(err) => println!("Could not modify .gitignore: {err}"),
    }
    cmd!(sh, "git add .").run()?;
    cmd!(sh, "git commit -m \"init\"").run()?;

    if opts.gh {
        let cwd = sh.current_dir();
        cmd!(
            sh,
            "gh repo create {script_name} --public --push --source={cwd}"
        )
        .run()?;
    };

    Ok(())
}
