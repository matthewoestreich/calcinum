use crate::value::conversion::ToPrimitive as _;

pub mod arithmetic;
pub mod conversion;
pub mod error;
pub mod fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    UnsignedInt(u64),
    UnsignedBigInt(u128),
    SignedInt(i64),
    SignedBigInt(i128),
    Float(f64),
}

impl Value {
    pub(crate) fn promote(&mut self) {
        *self = match *self {
            Value::UnsignedInt(n) => Self::UnsignedBigInt(n as _),
            Value::UnsignedBigInt(n) => {
                const SI_MAX: u128 = i64::MAX as _;
                const SBI_MIN: u128 = SI_MAX + 1;
                const SBI_MAX: u128 = i128::MAX as _;

                match n {
                    0..=SI_MAX => Self::SignedInt(n as _),
                    SBI_MIN..=SBI_MAX => Self::SignedBigInt(n as _),
                    _ => Self::Float(n.to_f64().expect("all u128 convert to f64")),
                }
            }
            Value::SignedInt(n) => Self::SignedBigInt(n as _),
            Value::SignedBigInt(n) => Self::Float(n.to_f64().expect("all i128 convert to f64")),
            Value::Float(n) => Self::Float(n),
        }
    }
}

// ======
// Order
// ======

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Order {
    UnsignedInt,
    UnsignedBigInt,
    SignedInt,
    SignedBigInt,
    Float,
}

impl From<Value> for Order {
    fn from(value: Value) -> Self {
        match value {
            Value::UnsignedInt(_) => Self::UnsignedInt,
            Value::UnsignedBigInt(_) => Self::UnsignedBigInt,
            Value::SignedInt(_) => Self::SignedInt,
            Value::SignedBigInt(_) => Self::SignedBigInt,
            Value::Float(_) => Self::Float,
        }
    }
}

impl From<&Value> for Order {
    fn from(value: &Value) -> Self {
        match value {
            Value::UnsignedInt(_) => Self::UnsignedInt,
            Value::UnsignedBigInt(_) => Self::UnsignedBigInt,
            Value::SignedInt(_) => Self::SignedInt,
            Value::SignedBigInt(_) => Self::SignedBigInt,
            Value::Float(_) => Self::Float,
        }
    }
}
