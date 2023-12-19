use bpaf::{Parser, Bpaf};
use xshell::{cmd, Shell};

// bpaf docs: https://docs.rs/bpaf/latest/bpaf/index.html
// xshell docs: https://docs.rs/xshell/latest/xshell/index.html

/// Template Rust CLI script.
#[derive(Bpaf, Debug, Clone)]
struct Template {
    /// Example of an optional flag.
    #[bpaf(short, long)]
    opt: bool,

    /// Example of an optional argument.
    #[bpaf(argument("OPTIONAL_ARG"), short, long)]
    arg: Option<usize>,

    /// Example of a positional argument.
    #[bpaf(positional("POSITIONAL"))]
    pos: String,
}

fn main() -> anyhow::Result<()> {
    let opts = template().run();
    let greeting = if opts.opt { "goodbye" } else { "hello" };
    let thing = opts.pos.repeat(opts.arg.unwrap_or(1));
    let message = format!("{greeting} {thing}!");
    let sh = Shell::new()?;
    cmd!(sh, "echo \"{message}\"").run()?;

    Ok(())
}
