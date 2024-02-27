pub mod error;
pub mod eval;
pub mod parse;
pub mod run;
pub mod cli;
pub mod values;

use crate::values::RsLispValue;

pub type Result<T> = anyhow::Result<T, anyhow::Error>;
pub type RsLispResult = Result<Box<RsLispValue>>;

