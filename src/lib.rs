mod ast;
mod calculator;
mod number;

pub use bigdecimal;
pub use calculator::*;
pub use num_bigint;
pub use number::{Formatting, Number, NumberOrder, ToNumber, error::NumberError};

/// Evaluates expression.
///
/// ```rust
/// use calcinum::{eval, Number};
///
/// assert_eq!(eval("1+1"), Ok(Number::from(2)));
/// ```
pub fn eval(expression: &str) -> Result<Number, CalculatorError> {
    let tokens = ast::tokenize(expression)?;
    let rpn_tokens = ast::parse(tokens)?;
    let result = ast::eval(rpn_tokens)?;
    Ok(result)
}
