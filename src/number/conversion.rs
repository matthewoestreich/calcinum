use super::{Number, error::NumberError};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use std::str::FromStr;

// ===========================================================================================
// ========================== ToNumber =======================================================
// ===========================================================================================

#[allow(dead_code)]
pub trait ToNumber {
    fn to_number(&self) -> Number;
}

macro_rules! impl_to_number {
    ($t:ty) => {
        impl ToNumber for $t {
            fn to_number(&self) -> Number {
                Number::from(*self)
            }
        }
    };
}

impl_to_number!(u8);
impl_to_number!(u16);
impl_to_number!(u32);
impl_to_number!(u64);
impl_to_number!(u128);
impl_to_number!(i8);
impl_to_number!(i16);
impl_to_number!(i32);
impl_to_number!(i64);
impl_to_number!(i128);

impl ToNumber for f64 {
    fn to_number(&self) -> Number {
        Number::from_f64(*self).expect("Number")
    }
}

impl ToNumber for BigInt {
    fn to_number(&self) -> Number {
        Number::from(self)
    }
}

impl ToNumber for BigDecimal {
    fn to_number(&self) -> Number {
        Number::from(self)
    }
}

// ===========================================================================================
// ========================== From ===========================================================
// ===========================================================================================

macro_rules! impl_number_from {
    ($t:ty) => {
        impl From<$t> for Number {
            fn from(value: $t) -> Self {
                Number::Int(BigInt::from(value))
            }
        }

        impl From<&$t> for Number
        where
            $t: Copy,
        {
            fn from(value: &$t) -> Self {
                Number::Int(BigInt::from(*value))
            }
        }
    };
}

impl_number_from!(u8);
impl_number_from!(u16);
impl_number_from!(u32);
impl_number_from!(u64);
impl_number_from!(u128);
impl_number_from!(i8);
impl_number_from!(i16);
impl_number_from!(i32);
impl_number_from!(i64);
impl_number_from!(i128);

impl From<BigDecimal> for Number {
    fn from(value: BigDecimal) -> Self {
        Number::Decimal(value)
    }
}

/// Clones the value!!
impl From<&BigDecimal> for Number {
    fn from(value: &BigDecimal) -> Self {
        Number::Decimal(value.clone())
    }
}

impl From<BigInt> for Number {
    fn from(value: BigInt) -> Self {
        Number::Int(value)
    }
}

/// Clones the value!!
impl From<&BigInt> for Number {
    fn from(value: &BigInt) -> Self {
        Number::Int(value.clone())
    }
}

// ===========================================================================================
// ========================== TryFrom ========================================================
// ===========================================================================================

impl TryFrom<f64> for Number {
    type Error = NumberError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        let bd = BigDecimal::from_str(&value.to_string())?;
        Ok(Number::Decimal(bd))
    }
}

// ===========================================================================================
// ========================== FromStr ========================================================
// ===========================================================================================

impl FromStr for Number {
    type Err = NumberError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<BigInt>().map(Self::Int).or_else(|_| {
            s.parse::<BigDecimal>()
                .map(Self::Decimal)
                .map_err(|_| NumberError::Parsing {
                    value: s.to_string(),
                })
        })
    }
}
