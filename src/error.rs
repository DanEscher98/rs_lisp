use thiserror::Error;
use rustyline::error::ReadlineError;
use std::{fmt::Debug, hash::Hash};

#[derive(Error, Debug)]
pub enum RsLispError {
    #[error("Divide by Zero")]
    DivideByZero,

    #[error("NaN: {0}")]
    NotANumber(#[from] std::num::ParseIntError),

    #[error("Empty list")]
    EmptyList,

    #[error("Function format invalid")]
    FunctionFormat,

    #[error("LispValue has no children")]
    NoChildren,

    #[error("Wrong number of arguments: expected {0}, received {1}")]
    NumArguments(usize, usize),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("ReadlineError: {0}")]
    ReadlineError(#[from] ReadlineError),

    #[error("Wrong type: expected {0}, received {1}")]
    WrongType(String, String),

    #[error("Unknown function: {0}")]
    UnknownFunction(String)
}

impl<T> From<pest::error::Error<T>> for RsLispError
where
    T: Debug + Ord + Copy + Hash
{
    fn from(error: pest::error::Error<T>) -> Self {
        RsLispError::ParseError(format!("{}", error))
    }
}

impl From<std::io::Error> for RsLispError {
    fn from(error: std::io::Error) -> Self {
        RsLispError::ParseError(error.to_string())
    }
}
