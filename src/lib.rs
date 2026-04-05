mod number;
mod shunting_yard;

pub use bigdecimal::BigDecimal;
pub use num_bigint::BigInt;
pub use number::{Number, NumberError, NumberOrder};

use bigdecimal::ParseBigDecimalError;
use std::{error, fmt};

pub fn parse_expression(expression: &str) -> Result<Number, CalculatorError> {
    shunting_yard::parse(expression)
}

#[derive(Debug, Clone)]
pub enum CalculatorError {
    ParseBigDecimal(ParseBigDecimalError),
    EmptyExpression,
    InvalidExpression,
    InvalidExponent { exponent_str: String },
    NumberError(NumberError),
}

impl fmt::Display for CalculatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalculatorError::InvalidExponent { exponent_str } => write!(
                f,
                "exponent : {exponent_str} : is either Number::Decimal(x) or is unable to be represented by an i64 (eg. it is a float, etc..)"
            ),
            CalculatorError::ParseBigDecimal(e) => write!(f, "error parsing BigDecimal : {e}"),
            CalculatorError::EmptyExpression => write!(f, "expression cannot be empty"),
            CalculatorError::InvalidExpression => {
                write!(f, "you may be missing a parenthesis or number somewhere")
            }
            CalculatorError::NumberError(ne) => write!(f, "{ne}"),
        }
    }
}

impl From<NumberError> for CalculatorError {
    fn from(error: NumberError) -> Self {
        Self::NumberError(error)
    }
}

impl From<ParseBigDecimalError> for CalculatorError {
    fn from(value: ParseBigDecimalError) -> Self {
        Self::ParseBigDecimal(value)
    }
}

impl error::Error for CalculatorError {}

#[derive(Debug, Clone)]
pub enum ExpressionError {
    InvalidOrMissingParenthesis,
}

impl fmt::Display for ExpressionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExpressionError::InvalidOrMissingParenthesis => {
                write!(f, "Expression is invalid or missing a parenthesis")
            }
        }
    }
}

impl error::Error for ExpressionError {}
