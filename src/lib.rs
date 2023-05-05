pub mod error;
pub mod eval;
pub mod parse;
pub mod run;
pub mod cli;
pub mod values;

use crate::values::LambdaValue;

pub type Result<T> = anyhow::Result<T, anyhow::Error>;
pub type LambdaResult = Result<Box<LambdaValue>>;

