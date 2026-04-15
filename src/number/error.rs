use astro_float::{Error as AstroError, Sign as AstroSign};
use bigdecimal::ParseBigDecimalError;
use num_bigint::ParseBigIntError;
use std::{error, fmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NumberError {
    Parsing { value: String },
    InvalidExponent { message: String },
    DivisionByZero,
    IsNaNOrInfinity,
    ExponentOverflow(AstroSign),
    InvalidArgument,
    MemoryAllocation,
}

impl fmt::Display for NumberError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NumberError::IsNaNOrInfinity => {
                write!(f, "cannot represent NaN or Infinity as a Number")
            }
            NumberError::Parsing { value } => write!(f, "Error parsing value : {value}"),
            NumberError::InvalidExponent { message } => write!(f, "{message}"),
            NumberError::DivisionByZero => write!(f, "attempt to divide by zero"),
            NumberError::ExponentOverflow(sign) => write!(f, "exponent overflow '{sign:?}'"),
            NumberError::InvalidArgument => write!(f, "invalid argument"),
            NumberError::MemoryAllocation => write!(f, "memory allocation failed"),
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

impl From<ParseBigIntError> for NumberError {
    fn from(value: ParseBigIntError) -> Self {
        Self::Parsing {
            value: value.to_string(),
        }
    }
}

impl From<AstroError> for NumberError {
    fn from(err: AstroError) -> Self {
        match err {
            AstroError::ExponentOverflow(sign) => Self::ExponentOverflow(sign),
            AstroError::DivisionByZero => Self::DivisionByZero,
            AstroError::InvalidArgument => Self::InvalidArgument,
            AstroError::MemoryAllocation => Self::MemoryAllocation,
        }
    }
}

impl error::Error for NumberError {}
