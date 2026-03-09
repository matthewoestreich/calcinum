pub mod arithmetic;
pub mod comparison;
pub mod conversion;
pub mod error;
pub mod fmt;
//pub mod numeric;

pub(crate) mod dispatch_operation;
pub(crate) use dispatch_operation::*;

use num_bigint::{BigInt, BigUint};
use num_traits::{FromPrimitive, ToPrimitive};
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub enum Value {
    UnsignedInt(u128),
    UnsignedBigInt(BigUint),
    SignedInt(i128),
    SignedBigInt(BigInt),
    Float(f64),
}

impl Value {
    /// Get the order of this value
    pub(crate) fn order(&self) -> Order {
        Order::from(self)
    }

    pub(crate) fn promote(&mut self) {
        *self = match self.clone() {
            Value::UnsignedInt(v) => {
                if v <= i128::MAX as u128 {
                    Value::SignedInt(v as i128)
                } else {
                    Value::UnsignedBigInt(BigUint::from(v))
                }
            }
            Value::SignedInt(v) => {
                if v >= 0 {
                    Value::UnsignedBigInt(BigUint::from(v as u128))
                } else {
                    Value::SignedBigInt(BigInt::from(v))
                }
            }
            Value::UnsignedBigInt(v) => Value::SignedBigInt(BigInt::from(v)),
            Value::SignedBigInt(v) => Value::Float(v.to_f64().unwrap_or(f64::INFINITY)),
            Value::Float(v) => Value::Float(v),
        };
    }

    /// Promote this value until it is signed, according to its value.
    pub(crate) fn promote_to_signed(&mut self) {
        while self.order() <= Order::UnsignedBigInt {
            self.promote();
        }
    }

    /// Promote this value until it is a float.
    pub(crate) fn promote_to_float(&mut self) -> &mut f64 {
        // there is no case where an integer value produces NaN when converted to a float
        *self = match self.clone() {
            Value::UnsignedInt(n) => (n as f64).into(),
            Value::UnsignedBigInt(n) => (n.to_f64()).expect("no error").into(),
            Value::SignedInt(n) => (n as f64).into(),
            Value::SignedBigInt(n) => (n.to_f64()).expect("no error").into(),
            Value::Float(n) => n.into(),
        };
        let Self::Float(f) = self else {
            unreachable!("we just promoted up to float")
        };
        f
    }

    /// Demote this value to the narrowest valid container type
    pub(crate) fn demote(&mut self) {
        *self = match self.clone() {
            Value::UnsignedBigInt(n) => {
                if let Some(v) = n.to_u128() {
                    Value::UnsignedInt(v)
                } else {
                    Value::UnsignedBigInt(n)
                }
            }
            Value::SignedBigInt(n) => {
                if let Some(v) = n.to_i128() {
                    Value::SignedInt(v)
                } else {
                    Value::SignedBigInt(n)
                }
            }
            Value::Float(f) if f.fract() == 0.0 => {
                if let Some(bi) = num_bigint::BigInt::from_f64(f) {
                    if let Some(v) = bi.to_i128() {
                        Value::SignedInt(v)
                    } else {
                        Value::SignedBigInt(bi)
                    }
                } else {
                    Value::Float(f)
                }
            }
            other => other,
        };
    }

    /// Find the minimum compatible order for `self` and `other` by promoting the lesser until they match.
    pub(crate) fn match_orders(&mut self, other: &mut Self) {
        while self.order() != other.order() {
            match self.order().cmp(&other.order()) {
                Ordering::Equal => unreachable!("orders already known not to be equal"),
                Ordering::Less => self.promote(),
                Ordering::Greater => other.promote(),
            }
        }
    }
}

// ======
// Order
// ======

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Order {
    UnsignedInt,
    SignedInt,
    UnsignedBigInt,
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

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(10_u8)]
    fn promotion(#[case] value: impl Into<Value>) {}
}
