use crate::value::Value;
use std::fmt::{Binary, Formatter, Result};

impl Binary for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Value::UnsignedInt(n) => Binary::fmt(n, f),
            Value::UnsignedBigInt(n) => Binary::fmt(n, f),
            Value::SignedInt(n) => Binary::fmt(n, f),
            Value::SignedBigInt(n) => Binary::fmt(n, f),
            Value::Float(n) => Binary::fmt(&n.to_bits(), f),
        }
    }
}
