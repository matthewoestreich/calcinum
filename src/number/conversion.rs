use crate::{Number, NumberError};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_traits::{Signed, ToPrimitive};
use std::str::FromStr;

impl Number {
    pub fn from_f64(n: f64) -> Result<Self, NumberError> {
        Self::try_from(n)
    }

    pub fn to_i64(&self) -> Option<i64> {
        match self {
            Number::Int(i) => i.to_i64(),
            Number::Decimal(d) => d.to_i64(),
        }
    }

    /// If `self` is `Number::Decimal` calling this method may result in data loss!
    /// This is due to how decimal to integer conversion works.
    /// IMPORTANT: if your number does not fit into an `i64`, it will be saturated,
    /// eg. clamped to `i64` bounds, which may result in data loss!
    pub fn to_i64_saturating(&self) -> i64 {
        match self {
            Number::Int(i) => Self::saturating_i64(i),
            Number::Decimal(d) => Self::saturating_i64(d),
        }
    }

    pub fn to_i32(&self) -> Option<i32> {
        match self {
            Number::Int(i) => i.to_i32(),
            Number::Decimal(d) => d.to_i32(),
        }
    }

    pub fn to_i128(&self) -> Option<i128> {
        match self {
            Number::Int(i) => i.to_i128(),
            Number::Decimal(d) => d.to_i128(),
        }
    }

    /// If `self` is `Number::Decimal` calling this method may result in data loss!
    /// This is due to how decimal to integer conversion works.
    /// IMPORTANT: if your number does not fit into an `i128`, it will be saturated,
    /// eg. clamped to `i128` bounds, which may result in data loss!
    pub fn to_i128_saturating(&self) -> i128 {
        match self {
            Number::Int(i) => Self::saturating_i128(i),
            Number::Decimal(d) => Self::saturating_i128(d),
        }
    }

    /// If the underlying value for `T` does not fit within an
    /// `i128`, we truncate it to fit within `i128` bounds, which
    /// may result in data/precision/scale loss!
    fn saturating_i128<T>(x: &T) -> i128
    where
        T: ToPrimitive + Signed,
    {
        x.to_i128().unwrap_or_else(|| {
            if x.signum().is_negative() {
                i128::MIN
            } else {
                i128::MAX
            }
        })
    }

    /// If the underlying value for `T` does not fit within an
    /// `i64`, we truncate it to fit within `i64` bounds, which
    /// may result in data/precision/scale loss!
    fn saturating_i64<T>(x: &T) -> i64
    where
        T: ToPrimitive + Signed,
    {
        x.to_i64().unwrap_or_else(|| {
            if x.signum().is_negative() {
                i64::MIN
            } else {
                i64::MAX
            }
        })
    }
}

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

// ===========================================================================================
// ========================== Tests ==========================================================
// ===========================================================================================

#[cfg(test)]
mod test {
    use crate::*;
    use rstest::*;
    use std::str::FromStr as _;

    #[rstest]
    #[case::from_str1("1.1", "1.1", NumberOrder::Decimal)]
    #[case::from_str2("1", "1", NumberOrder::Int)]
    fn from_str(#[case] number: &str, #[case] expect_str: &str, #[case] expect_order: NumberOrder) {
        let x = Number::from_str(number).unwrap();
        // for now use impl Display for assertion
        let xr = format!("{x}");
        assert_eq!(xr, expect_str, "expected str '{expect_str}' got str '{xr}'");
        let xo = x.order();
        assert_eq!(
            xo, expect_order,
            "expected order '{expect_order:?}' got order '{xo:?}'"
        );
    }

    #[test]
    fn from_f64() {
        let a = Number::from_f64(1.1).unwrap();
        assert_eq!(a.order(), NumberOrder::Decimal);
    }
}
