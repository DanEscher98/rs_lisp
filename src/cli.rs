use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Default, Debug)]
#[clap(
    author="Danyiel Colin",
    version="0.1",
    about="A simple lisp-like repl"
)]
pub struct Arguments {
    #[arg(short, long, action)]
    debug: bool,

    #[arg(short, long)]
    input: Option<PathBuf>
}
