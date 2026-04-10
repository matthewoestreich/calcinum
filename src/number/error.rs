use bigdecimal::ParseBigDecimalError;
use std::{error, fmt};

#[derive(Debug, Clone)]
pub enum NumberError {
    Parsing { value: String },
    InvalidExponent { message: String },
}

impl fmt::Display for NumberError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NumberError::Parsing { value } => write!(f, "Error parsing value : {value}"),
            NumberError::InvalidExponent { message } => write!(f, "{message}"),
        }
    }
}

impl From<ParseBigDecimalError> for NumberError {
    fn from(value: ParseBigDecimalError) -> Self {
        Self::Parsing {
            value: value.to_string(),
        }
    }
}

impl error::Error for NumberError {}
