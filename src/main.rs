use bpaf::{self, Parser, Bpaf};
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
}

fn main() -> anyhow::Result<()> {
    let opts = mk_script().run();
    let script_name = opts.script_name;

    let templates =     Templates {
        cli_main_rs: include_str!("template_cli.rs"),
        shell_main_rs: include_str!("template_shell.rs"),
        default_nix: |script_name| include_str!("../default.nix").replace("TEMPLATE_PLACEHOLDER", script_name),
        test_build_nix: include_str!("../test_build.nix"),
    };

    let sh = Shell::new()?;
    // TODO: if project already exists, ask if it should be deleted?
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
    sh.write_file("./src/main.rs", if opts.cli {
        templates.cli_main_rs
    } else {
        templates.shell_main_rs
    })?;
    sh.write_file("./default.nix", (templates.default_nix)(&script_name))?;
    sh.write_file("./test_build.nix", templates.test_build_nix)?;
    cmd!(sh, "rustfmt ./src/main.rs").run()?;

    let interact_with_reuse = format!("Apache-2.0\nMIT\n\n{script_name}\nhttps://github.com/bzm3r/{script_name}\nBrian Merchant\nbzm3r@proton.me\n");
    cmd!(sh, "reuse init").stdin(&interact_with_reuse).run()?;

    cmd!(sh, "git init").run()?;
    let original_contents = sh.read_file(".gitignore")?;
    sh.write_file(".gitignore", format!("{original_contents}/result\n"))?;
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
