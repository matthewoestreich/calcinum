use crate::ast::error::ParserError;
use std::{fmt, str::FromStr};
use varienum::VariantsVec;

///
/// -- Important info --
///
/// `round` : We round to nearest integer (0 decimal places); e.g., `12345.9448820304` -> `12346` and `12345.4448820304` -> `12345`.
///           Whole numbers are just returned as is; e.g., `12345` -> `12345` and `69420` -> `69420`.
///           Rounding mode is half even; round to ‘nearest neighbor’, if equidistant, round towards nearest even digit.
///

#[derive(Debug, Clone, VariantsVec)]
pub enum Function {
    #[description = "abs"]
    Abs,
    #[description = "floor"]
    Floor,
    #[description = "ceil"]
    Ceil,
    #[description = "sin"]
    Sin,
    #[description = "cos"]
    Cos,
    #[description = "tan"]
    Tan,
    #[description = "round"]
    Round,
    #[description = "sinh"]
    Sinh,
    #[description = "cosh"]
    Cosh,
    #[description = "tanh"]
    Tanh,
    #[description = "rad"]
    Rad,
    #[description = "sqrt"]
    Sqrt,
}

//
// IF YOU'RE ADDING A NEW FUNCTION, DON'T FORGET
// TO ADD IT TO THE `from_str` MATCH BELOW!!!
//
impl FromStr for Function {
    type Err = ParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "abs" => Self::Abs,
            "floor" => Self::Floor,
            "ceil" => Self::Ceil,
            "sin" => Self::Sin,
            "cos" => Self::Cos,
            "tan" => Self::Tan,
            "round" => Self::Round,
            "sinh" => Self::Sinh,
            "cosh" => Self::Cosh,
            "tanh" => Self::Tanh,
            "rad" => Self::Rad,
            "sqrt" => Self::Sqrt,
            _ => {
                return Err(ParserError::UnrecognizedFunction {
                    name: s.to_string(),
                });
            }
        })
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            //
            // All functions should be lower case!
            //
            Function::Abs => write!(f, "abs"),
            Function::Floor => write!(f, "floor"),
            Function::Ceil => write!(f, "ceil"),
            Function::Sin => write!(f, "sin"),
            Function::Cos => write!(f, "cos"),
            Function::Tan => write!(f, "tan"),
            Function::Round => write!(f, "round"),
            Function::Sinh => write!(f, "sinh"),
            Function::Cosh => write!(f, "cosh"),
            Function::Tanh => write!(f, "tanh"),
            Function::Rad => write!(f, "rad"),
            Function::Sqrt => write!(f, "sqrt"),
        }
    }
}
