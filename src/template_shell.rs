use xshell::{cmd, Shell};

// xshell docs: https://docs.rs/xshell/latest/xshell/index.html

fn main() -> anyhow::Result<()> {
    let sh = Shell::new()?;

    cmd!(sh, "echo \"hello world!\"").run()?;

    Ok(())
}
