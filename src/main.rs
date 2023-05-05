use clap::Parser;
use rs_lisp::{
    cli::Arguments,
    run::run,
    Result
};


fn main() -> Result<()> {
    run(Arguments::parse())
}
