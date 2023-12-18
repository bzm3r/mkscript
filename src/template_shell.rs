use xshell::{cmd, Shell};

fn main() -> anyhow::Result<()> {
    let sh = Shell::new()?;

    cmd!(sh, "echo \"hello world!\"").run()?;

    Ok(())
}
