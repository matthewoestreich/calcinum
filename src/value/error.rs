use crate::value::Value;
use std::{error, fmt};

#[derive(Debug, Clone)]
pub enum Error {
    Converting { from: Value, to: String },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Converting { from, to } => {
                write!(f, "Cannot convert {:?} to {:?}", from, to)
            }
        }
    }
}

impl error::Error for Error {}
