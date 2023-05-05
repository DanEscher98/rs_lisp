use crate::cli::Arguments;
use crate::Result;

pub fn run(args: Arguments) -> Result<()> {
    println!("{:?}", args);
    Ok(())
}
