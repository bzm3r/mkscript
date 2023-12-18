use bpaf::{self, construct, positional, OptionParser, Parser, short};
use xshell::{cmd, Shell};

#[derive(Debug, Clone)]
struct Interface {
    /// Example of a positional argument.
    pos: String,

    /// Example of an optional flag.
    opt: bool,

    /// Example of an optional argument.
    arg: Option<usize>,
}

fn opts() -> OptionParser<Interface> {
    let pos = positional("POSITIONAL").help("Example of a positional argument.");
    let opt = short('o').long("opt")
        .help("Example of an optional flag..")
        .switch();
    let arg = short('a').long("arg")
        .help("Example of an optional argument.")
        .argument("OPTIONAL_ARG").optional();
    construct!(Interface {
        pos,
        opt,
        arg
    })
    .to_options()
    .descr("Template Rust CLI script.")
}

fn main() -> anyhow::Result<()> {
    let opts = opts().run();
    let greeting = if opts.opt { "goodbye" } else { "hello" };
    let thing = opts.pos.repeat(opts.arg.unwrap_or(1));
    let message = format!("{greeting} {thing}!");
    let sh = Shell::new()?;
    cmd!(sh, "echo \"{message}\"").run()?;

    Ok(())
}
