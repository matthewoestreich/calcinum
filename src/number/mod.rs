pub mod error;

#[macro_use]
mod macros;
mod arithmetic;
mod bitwise;
mod comparison;
mod conversion;

use bigdecimal::BigDecimal;
use error::NumberError;
use num_bigint::BigInt;
use num_traits::{Signed, ToPrimitive};
use std::{cmp::Ordering, fmt};

#[derive(Clone)]
pub enum Number {
    Int(BigInt),
    Decimal(BigDecimal),
}

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

    /// Sets the scale only on Number::Decimal
    pub fn set_scale(&mut self, scale: i64) {
        if let Self::Decimal(n) = self {
            *n = n.with_scale(scale);
        }
    }

    /// Sets the scale and rounding mode only on Number::Decimal
    pub fn set_scale_round(&mut self, scale: i64, rounding_mode: bigdecimal::RoundingMode) {
        if let Self::Decimal(n) = self {
            *n = n.with_scale_round(scale, rounding_mode);
        }
    }

    pub fn order(&self) -> NumberOrder {
        NumberOrder::from(self)
    }

    pub fn match_order(&mut self, other: &mut Self) {
        match self.order().cmp(&other.order()) {
            Ordering::Less => self.promote(),
            Ordering::Greater => other.promote(),
            Ordering::Equal => {}
        }
    }

    /// Converts Number::Int to Number::Decimal.
    /// Number::Decimal is already the highest 'order'.
    pub fn promote(&mut self) {
        if let Some(n) = self.take_int() {
            *self = Self::Decimal(BigDecimal::from(n));
        }
    }

    /// Converts `Number::Decimal` to `Number::Int`.
    /// IMPORTANT : this may cause loss of data/precision!
    pub fn demote(&mut self) {
        if let Some(ref mut d) = self.take_decimal() {
            let (d, _) = d.with_scale(0).into_bigint_and_scale();
            *self = Self::Int(d);
        }
    }

    /// Takes the backing BigInt leaivng 0 in it's place.
    /// Returns None if variant isn't Number::Int
    pub fn take_int(&mut self) -> Option<BigInt> {
        if let Self::Int(n) = self {
            return Some(std::mem::take(n));
        }
        None
    }

    /// Takes the backing BigDecimal leaving 0 in it's place.
    /// Returns None if variant isn't Number::Decimal
    pub fn take_decimal(&mut self) -> Option<BigDecimal> {
        if let Self::Decimal(d) = self {
            return Some(std::mem::take(d));
        }
        None
    }

    /// If variant is `Number::Decimal` we return the integer part is binary
    /// and the fractional part as binary, separated by a period.
    /// For example, if you have a `Number::Decimal(100.773)` this method
    /// returns : `"1100100.1100000101"`
    pub fn to_binary_str(&self) -> String {
        match self {
            Number::Int(big_int) => format!("{big_int:b}").to_string(),
            Number::Decimal(big_decimal) => {
                let s = big_decimal.to_string();
                let parts: Vec<_> = s.split('.').collect();
                let mut output = Self::to_bin_str(parts[0]);
                if parts[1].is_empty() {
                    output
                } else {
                    output.push('.');
                    output.push_str(&Self::to_bin_str(parts[1]));
                    output
                }
            }
        }
    }

    // ===========================================================================================
    // ========================== Mathematical Functions =========================================
    // ===========================================================================================

    pub fn pow(&self, exponent: i64) -> Result<Self, NumberError> {
        match self {
            Number::Decimal(d) => Ok(Number::Decimal(d.powi(exponent))),
            Number::Int(i) => {
                let exponent_u32: u32 = exponent.try_into().map_err(|_| {
                    let m = format!("Number::Int exponent must fit in u32: {exponent} does not!");
                    NumberError::InvalidExponent { message: m }
                })?;
                Ok(Number::Int(i.pow(exponent_u32)))
            }
        }
    }

    /// The distance of a number from zero on a number line, regardless of direction.
    /// As a distance, it is always non-negative, effectively turning negative numbers
    /// positive and leaving positive numbers (and zero) unchanged.
    pub fn abs(&self) -> Self {
        match self {
            Number::Int(i) => Number::Int(i.abs()),
            Number::Decimal(d) => Number::Decimal(d.abs()),
        }
    }

    /// Variant is not coerced. If you call `.ceil()` with variant `Number::Int`,
    /// we just clone it and return it. If you call `.ceil()` on variant `Number::Decimal`,
    /// even though the result is a whole number, we keep it as a `Number::Decimal`.
    pub fn ceil(&self) -> Self {
        match self {
            Number::Int(_) => self.clone(),
            Number::Decimal(d) => {
                let bd = d.with_scale_round(0, bigdecimal::RoundingMode::Ceiling);
                Number::Decimal(bd)
            }
        }
    }

    /// Variant is not coerced. If you call `.floor()` with variant `Number::Int`,
    /// we just clone it and return it. If you call `.floor()` on variant `Number::Decimal`,
    /// even though the result is a whole number, we keep it as a `Number::Decimal`.
    pub fn floor(&self) -> Self {
        match self {
            Number::Int(_) => self.clone(),
            Number::Decimal(d) => {
                let bd = d.with_scale_round(0, bigdecimal::RoundingMode::Floor);
                Number::Decimal(bd)
            }
        }
    }

    // ===========================================================================================
    // ========================== Static Methods =================================================
    // ===========================================================================================

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

    fn to_bin_str(decimal_str: &str) -> String {
        if decimal_str == "0" || decimal_str.is_empty() {
            return "0".to_string();
        }
        let is_negative = decimal_str.starts_with('-');
        let decimal_str = decimal_str.trim_start_matches('-');
        let mut digits = Vec::with_capacity(decimal_str.len());
        for c in decimal_str.chars() {
            if let Some(d) = c.to_digit(10) {
                digits.push(d as u8);
            } else {
                return format!("<INVALID_DIGIT_FOUND = '{c}'>");
            }
        }
        let mut binary_bits = String::new();
        while !digits.is_empty() {
            let mut remainder = 0;
            let mut next_digits = Vec::with_capacity(digits.len());
            // Long division by 2
            for &digit in &digits {
                let current = digit + remainder * 10;
                let quotient = current / 2;
                remainder = current % 2;
                // Only push if it's not a leading zero
                if !next_digits.is_empty() || quotient > 0 {
                    next_digits.push(quotient);
                }
            }
            // The remainder of the full division is our binary digit
            binary_bits.push(if remainder == 0 { '0' } else { '1' });
            digits = next_digits;
        }
        if is_negative {
            binary_bits.push('-');
        }
        // Reverse to get the correct order (MSB first)
        binary_bits.chars().rev().collect()
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::Int(big_int) => write!(f, "{big_int}"),
            Number::Decimal(big_decimal) => write!(f, "{big_decimal}"),
        }
    }
}

impl fmt::Debug for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(i) => write!(f, "Number::Int({i})"),
            Self::Decimal(d) => write!(f, "Number::Decimal({d})"),
        }
    }
}

impl fmt::Binary for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_binary_str())
    }
}

// ===========================================================================================
// ========================== NumberOrder ====================================================
// ===========================================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum NumberOrder {
    Int,
    Decimal,
}

impl From<Number> for NumberOrder {
    fn from(value: Number) -> Self {
        match value {
            Number::Int(_) => Self::Int,
            Number::Decimal(_) => Self::Decimal,
        }
    }
}

impl From<&Number> for NumberOrder {
    fn from(value: &Number) -> Self {
        match value {
            Number::Int(_) => Self::Int,
            Number::Decimal(_) => Self::Decimal,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{number::conversion::ToNumber, *};
    use rstest::*;
    use std::str::FromStr;

    #[test]
    fn from_str() {
        let a = Number::from_str("1.1").unwrap();
        let ea = 1.1.to_number();
        assert_eq!(a, ea, "expected {ea:?} got {a:?}");

        let b = Number::from_str("1").unwrap();
        let eb = 1.to_number();
        assert_eq!(b, eb, "expected {eb:?} got {b:?}");
    }

    #[test]
    fn from_f64() {
        let a = Number::from_f64(1.1).unwrap();
        assert_eq!(a.order(), NumberOrder::Decimal);
    }

    #[rstest]
    #[case::binary_str1(
        "17958432089245743489.3597843208120587934",
        "1111100100111001001010101101011001011010011101111111100110000001.11000111101110000110110101010111101001100101000101011010011110"
    )]
    #[case::binary_str_bigdecimal_neg(
        "-17958432089245743489.3597843208120587934",
        "-1111100100111001001010101101011001011010011101111111100110000001.11000111101110000110110101010111101001100101000101011010011110"
    )]
    #[case::binary_str2(
        "17958432089245743489",
        "1111100100111001001010101101011001011010011101111111100110000001"
    )]
    #[case::binary_str_bigint_neg(
        "-17958432089245743489",
        "-1111100100111001001010101101011001011010011101111111100110000001"
    )]
    fn binary_str(#[case] number: &str, #[case] expect: &str) {
        let n = Number::from_str(number).unwrap();
        let fr = format!("{n:b}");
        assert_eq!(
            expect, fr,
            "[format!(\"{n:b}\")] expected '{expect}' got '{fr}'"
        );
        let br = n.to_binary_str();
        assert_eq!(
            expect, br,
            "[n.to_binary_str()] expected '{expect}' got '{br}'"
        );
    }

    #[rstest]
    #[case::add1("1", "1", "2")]
    #[case::add2("1.1", "2.2", "3.3")]
    #[case::add3("1.1", "2", "3.1")]
    #[case::add4("2", "1.1", "3.1")]
    fn add(#[case] lhs: &str, #[case] rhs: &str, #[case] expect: &str) {
        let x = Number::from_str(lhs).unwrap();
        let y = Number::from_str(rhs).unwrap();
        let e = Number::from_str(expect).unwrap();
        let r = x + y;
        assert_eq!(r, e, "expected {e:?} got {r:?}");
    }

    #[rstest]
    #[case::add_assign1("1", "1", "2")]
    #[case::add_assign2("1.1", "2.2", "3.3")]
    #[case::add_assign3("1.1", "2", "3.1")]
    #[case::add_assign4("2", "1.1", "3.1")]
    fn add_assign(#[case] lhs: &str, #[case] rhs: &str, #[case] expect: &str) {
        let mut x = Number::from_str(lhs).unwrap();
        let y = Number::from_str(rhs).unwrap();
        let e = Number::from_str(expect).unwrap();
        x += y;
        assert_eq!(x, e, "expected {e:?} got {x:?}");
    }

    #[rstest]
    #[case::sub1("1", "1", "0")]
    #[case::sub2("1.1", "2.2", "-1.1")]
    #[case::sub3("2", "1.1", "0.9")]
    #[case::sub4("100", "47.4567", "52.5433")]
    #[case::sub5("5.5", "2.2", "3.3")]
    fn sub(#[case] lhs: &str, #[case] rhs: &str, #[case] expect: &str) {
        let x = Number::from_str(lhs).unwrap();
        let y = Number::from_str(rhs).unwrap();
        let e = Number::from_str(expect).unwrap();
        let r = x - y;
        assert_eq!(r, e, "expected {e:?} got {r:?}");
    }

    #[rstest]
    #[case::sub_assign1("1", "1", "0")]
    #[case::sub_assign2("1.1", "2.2", "-1.1")]
    #[case::sub_assign3("2", "1.1", "0.9")]
    #[case::sub_assign4("100", "47.4567", "52.5433")]
    #[case::sub_assign5("5.5", "2.2", "3.3")]
    fn sub_assign(#[case] lhs: &str, #[case] rhs: &str, #[case] expect: &str) {
        let mut x = Number::from_str(lhs).unwrap();
        let y = Number::from_str(rhs).unwrap();
        let e = Number::from_str(expect).unwrap();
        x -= y;
        assert_eq!(x, e, "expected {e:?} got {x:?}");
    }

    #[rstest]
    #[case::mul1("1", "1", "1")]
    #[case::mul2("1.1", "2.2", "2.42")]
    #[case::mul3("2", "1.1", "2.2")]
    #[case::mul4("47.4567", "100", "4745.67")]
    #[case::mul5("55", "22", "1210")]
    #[case::mul6("5.7", "2", "11.4")]
    fn mul(#[case] lhs: &str, #[case] rhs: &str, #[case] expect: &str) {
        let x = Number::from_str(lhs).unwrap();
        let y = Number::from_str(rhs).unwrap();
        let e = Number::from_str(expect).unwrap();
        let r = x * y;
        assert_eq!(r, e, "expected {e:?} got {r:?}");
    }

    #[rstest]
    #[case::mul_assign1("1", "1", "1")]
    #[case::mul_assign2("1.1", "2.2", "2.42")]
    #[case::mul_assign3("2", "1.1", "2.2")]
    #[case::mul_assign4("47.4567", "100", "4745.67")]
    #[case::mul_assign5("55", "22", "1210")]
    #[case::mul_assign6("5.7", "2", "11.4")]
    fn mul_assign(#[case] lhs: &str, #[case] rhs: &str, #[case] expect: &str) {
        let mut x = Number::from_str(lhs).unwrap();
        let y = Number::from_str(rhs).unwrap();
        let e = Number::from_str(expect).unwrap();
        x *= y;
        assert_eq!(x, e, "expected {e:?} got {x:?}");
    }

    #[rstest]
    #[case::div1("1", "1", "1")]
    #[case::div2("1.1", "2.2", "0.5")]
    #[case::div3("2", "1.1", "1.81818181818")]
    #[case::div4("100", "47", "2.12765957447")]
    #[case::div5("55", "5", "11")]
    #[case::div6("5.7", "2", "2.85")]
    fn div(#[case] lhs: &str, #[case] rhs: &str, #[case] expect: &str) {
        let x = Number::from_str(lhs).unwrap();
        let y = Number::from_str(rhs).unwrap();
        let e = Number::from_str(expect).unwrap();
        let mut r = x / y;
        r.set_scale_round(11, bigdecimal::RoundingMode::HalfUp);
        assert_eq!(r, e, "expected {e:?} got {r:?}");
    }

    #[rstest]
    #[case::div_assign1("1", "1", "1")]
    #[case::div_assign2("1.1", "2.2", "0.5")]
    #[case::div_assign3("2", "1.1", "1.81818181818")]
    #[case::div_assign4("100", "47", "2.12765957447")]
    #[case::div_assign5("55", "5", "11")]
    #[case::div_assign6("5.7", "2", "2.85")]
    fn div_assign(#[case] lhs: &str, #[case] rhs: &str, #[case] expect: &str) {
        let mut x = Number::from_str(lhs).unwrap();
        let y = Number::from_str(rhs).unwrap();
        let e = Number::from_str(expect).unwrap();
        x /= y;
        x.set_scale_round(11, bigdecimal::RoundingMode::HalfUp);
        assert_eq!(x, e, "expected {e:?} got {x:?}");
    }

    #[rstest]
    #[case::rem1("1", "1", "0")]
    #[case::rem2("1.1", "2.2", "1.1")]
    #[case::rem3("2", "1.1", "0.9")]
    #[case::rem4("100", "47", "6")]
    #[case::rem5("55", "5", "0")]
    #[case::rem6("5.7", "2", "1.7")]
    #[case::rem7("5.6", "3.2", "2.4")]
    #[case::rem8("5.6", "2", "1.6")]
    fn rem(#[case] lhs: &str, #[case] rhs: &str, #[case] expect: &str) {
        let x = Number::from_str(lhs).unwrap();
        let y = Number::from_str(rhs).unwrap();
        let e = Number::from_str(expect).unwrap();
        let r = x % y;
        assert_eq!(r, e, "expected {e:?} got {r:?}");
    }

    #[rstest]
    #[case::rem_assign1("1", "1", "0")]
    #[case::rem_assign2("1.1", "2.2", "1.1")]
    #[case::rem_assign3("2", "1.1", "0.9")]
    #[case::rem_assign4("100", "47", "6")]
    #[case::rem_assign5("55", "5", "0")]
    #[case::rem_assign6("5.7", "2", "1.7")]
    #[case::rem_assign7("5.6", "3.2", "2.4")]
    #[case::rem_assign8("5.6", "2", "1.6")]
    fn rem_assign(#[case] lhs: &str, #[case] rhs: &str, #[case] expect: &str) {
        let mut x = Number::from_str(lhs).unwrap();
        let y = Number::from_str(rhs).unwrap();
        let e = Number::from_str(expect).unwrap();
        x %= y;
        assert_eq!(x, e, "expected {e:?} got {x:?}");
    }

    #[rstest]
    #[case::bitxor1("55", "84", "99")]
    #[case::bitxor2("57.284", "98.345", "91")]
    fn bitxor(#[case] lhs: &str, #[case] rhs: &str, #[case] expect: &str) {
        let x = Number::from_str(lhs).unwrap();
        let y = Number::from_str(rhs).unwrap();
        let e = Number::from_str(expect).unwrap();
        let r = x ^ y;
        assert_eq!(r, e, "expected {e:?} got {r:?}");
    }

    #[rstest]
    #[case::bitxor_assign1("55", "84", "99")]
    #[case::bitxor_assign2("57.284", "98.345", "91")]
    fn bitxor_assign(#[case] lhs: &str, #[case] rhs: &str, #[case] expect: &str) {
        let mut x = Number::from_str(lhs).unwrap();
        let y = Number::from_str(rhs).unwrap();
        let e = Number::from_str(expect).unwrap();
        x ^= y;
        assert_eq!(x, e, "expected {e:?} got {x:?}");
    }

    #[rstest]
    #[case::bitand1("55", "84", "20")]
    #[case::bitand2("55.4", "77.475", "5")]
    fn bitand(#[case] lhs: &str, #[case] rhs: &str, #[case] expect: &str) {
        let x = Number::from_str(lhs).unwrap();
        let y = Number::from_str(rhs).unwrap();
        let e = Number::from_str(expect).unwrap();
        let r = x & y;
        assert_eq!(r, e, "expected {e:?} got {r:?}");
    }

    #[rstest]
    #[case::bitand_assign1("55", "84", "20")]
    #[case::bitand_assign2("55.4", "77.475", "5")]
    fn bitand_assign(#[case] lhs: &str, #[case] rhs: &str, #[case] expect: &str) {
        let mut x = Number::from_str(lhs).unwrap();
        let y = Number::from_str(rhs).unwrap();
        let e = Number::from_str(expect).unwrap();
        x &= y;
        assert_eq!(x, e, "expected {e:?} got {x:?}");
    }

    #[rstest]
    #[case::bitor1("55", "84", "119")]
    #[case::bitor2(
        "97014118346046923173168730371588434847849321057273236539018427",
        "56473890472713285943048728314",
        "97014118346046923173168730371588439898750848355010217494179579"
    )]
    #[case::bitor3("55.432", "84.2113485", "119")]
    fn bitor(#[case] lhs: &str, #[case] rhs: &str, #[case] expect: &str) {
        let x = Number::from_str(lhs).unwrap();
        let y = Number::from_str(rhs).unwrap();
        let e = Number::from_str(expect).unwrap();
        let r = x | y;
        assert_eq!(r, e, "expected {e:?} got {r:?}");
    }

    #[rstest]
    #[case::bitor_assign1("55", "84", "119")]
    #[case::bitor_assign2(
        "97014118346046923173168730371588434847849321057273236539018427",
        "56473890472713285943048728314",
        "97014118346046923173168730371588439898750848355010217494179579"
    )]
    #[case::bitor_assign3("55.432", "84.2113485", "119")]
    fn bitor_assign(#[case] lhs: &str, #[case] rhs: &str, #[case] expect: &str) {
        let mut x = Number::from_str(lhs).unwrap();
        let y = Number::from_str(rhs).unwrap();
        let e = Number::from_str(expect).unwrap();
        x |= y;
        assert_eq!(x, e, "expected {e:?} got {x:?}");
    }

    #[rstest]
    #[case::shl1("55", "8", "14080")]
    #[case::shl2(
        "9701411834604692317316873037158843484784932105727",
        "2",
        "38805647338418769269267492148635373939139728422908"
    )]
    #[case::shl_lhs_decimal("10.5", "2", "40")]
    #[case::shl_lhs_decimal("10.534", "2.234", "40")]
    fn shl(#[case] lhs: &str, #[case] rhs: &str, #[case] expect: &str) {
        let x = Number::from_str(lhs).unwrap();
        let y = Number::from_str(rhs).unwrap();
        let e = Number::from_str(expect).unwrap();
        let r = x << y;
        assert_eq!(r, e, "expected {e:?} got {r:?}");
    }

    #[rstest]
    #[case::shl_assign1("55", "8", "14080")]
    #[case::shl_assign2(
        "9701411834604692317316873037158843484784932105727",
        "2",
        "38805647338418769269267492148635373939139728422908"
    )]
    #[case::shl_assign_lhs_decimal("10.5", "2", "40")]
    #[case::shl_assign_lhs_decimal("10.534", "2.234", "40")]
    fn shl_assign(#[case] lhs: &str, #[case] rhs: &str, #[case] expect: &str) {
        let mut x = Number::from_str(lhs).unwrap();
        let y = Number::from_str(rhs).unwrap();
        let e = Number::from_str(expect).unwrap();
        x <<= y;
        assert_eq!(x, e, "expected {e:?} got {x:?}");
    }

    #[rstest]
    #[case::shr1("873", "5", "27")]
    #[case::shr2(&i128::MAX.to_string(), "2", "42535295865117307932921825928971026431")]
    #[case::shr_lhs_gt_i128_max(
        "34028236692093846346337460743176821145434832943245",
        "2",
        "8507059173023461586584365185794205286358708235811"
    )]
    fn shr(#[case] lhs: &str, #[case] rhs: &str, #[case] expect: &str) {
        let x = Number::from_str(lhs).unwrap();
        let y = Number::from_str(rhs).unwrap();
        let e = Number::from_str(expect).unwrap();
        let r = x >> y;
        assert_eq!(r, e, "expected {e:?} got {r:?}");
    }

    #[rstest]
    #[case::shr_assign1("873", "5", "27")]
    #[case::shr_assign2(&i128::MAX.to_string(), "2", "42535295865117307932921825928971026431")]
    #[case::shr_lhs_gt_i128_max(
        "34028236692093846346337460743176821145434832943245",
        "2",
        "8507059173023461586584365185794205286358708235811"
    )]
    fn shr_assign(#[case] lhs: &str, #[case] rhs: &str, #[case] expect: &str) {
        let mut x = Number::from_str(lhs).unwrap();
        let y = Number::from_str(rhs).unwrap();
        let e = Number::from_str(expect).unwrap();
        x >>= y;
        assert_eq!(x, e, "expected {e:?} got {x:?}");
    }

    #[rstest]
    #[case::not1("55", "-56")]
    #[case::not2(
        "97014118346046923173168730371588434847849321057273236539018427",
        "-97014118346046923173168730371588434847849321057273236539018428"
    )]
    #[case::not3("55.432", "-56")]
    fn not(#[case] lhs: &str, #[case] expect: &str) {
        let x = Number::from_str(lhs).unwrap();
        let e = Number::from_str(expect).unwrap();
        let rr = !&x;
        assert_eq!(rr, e, "[by ref] expected {e:?}, got {rr:?}");
        let r = !x;
        assert_eq!(r, e, "[by val] expected {e:?} got {r:?}");
    }

    #[rstest]
    #[case::neg1("55", "-55")]
    #[case::neg2("55.55", "-55.55")]
    fn neg(#[case] number: &str, #[case] expect: &str) {
        let n = Number::from_str(number).unwrap();
        let e = Number::from_str(expect).unwrap();
        let r = -n;
        assert_eq!(r, e, "expected {e:?} got {r:?}");
    }

    #[rstest]
    #[case::abs1("10", "10")]
    #[case::abs1_1("10.123", "10.123")]
    #[case::abs2("-10", "10")]
    #[case::abs2_1("-10.123", "10.123")]
    #[case::abs3("0", "0")]
    #[case::abs3_1("-0", "0")]
    fn abs(#[case] n: &str, #[case] expect: &str) {
        let x = n.parse::<Number>().unwrap();
        let e = expect.parse::<Number>().unwrap();
        let r = x.abs();
        assert_eq!(r, e, "expected {e:?} got {r:?}");
    }

    #[rstest]
    #[case::ceil1("14.7572", "15")]
    #[case::ceil2("0.1", "1")]
    #[case::ceil3("-2.3", "-2")]
    #[case::ceil4("-0.9", "0")]
    #[case::ceil5("-7.5", "-7")]
    #[case::ceil6("5.0", "5")]
    #[case::ceil7("-4.0", "-4")]
    #[case::ceil8("0.0", "0")]
    #[case::ceil9("-0.0", "-0")]
    #[case::ceil10(
        "0.0000000000000000000000000000000000000000000000000000000000000000000000001",
        "1"
    )]
    #[case::ceil11(
        "-0.0000000000000000000000000000000000000000000000000000000000000000000000001",
        "0"
    )]
    fn ceil(#[case] n: &str, #[case] expect: &str) {
        let x = n.parse::<Number>().unwrap();
        let bd = expect.parse::<BigDecimal>().unwrap();
        let e = Number::from(bd);
        let r = x.ceil();
        assert_eq!(r, e, "expected {e:?} got {r:?}");
    }

    #[rstest]
    #[case::flor1("14.7572", "14")]
    #[case::flor2("0.1", "0")]
    #[case::flor3("-2.3", "-3")]
    #[case::flor4("-0.9", "-1")]
    #[case::flor5("-7.5", "-8")]
    #[case::flor6("5.0", "5")]
    #[case::flor7("-4.0", "-4")]
    #[case::flor8("0.0", "0")]
    #[case::flor9("-0.0", "-0")]
    #[case::floor10(
        "0.0000000000000000000000000000000000000000000000000000000000000000000000001",
        "0"
    )]
    #[case::floor11(
        "-0.0000000000000000000000000000000000000000000000000000000000000000000000001",
        "-1"
    )]
    fn floor(#[case] n: &str, #[case] expect: &str) {
        let x = n.parse::<Number>().unwrap();
        let bd = expect.parse::<BigDecimal>().unwrap();
        let e = Number::from(bd);
        let r = x.floor();
        assert_eq!(r, e, "expected {e:?} got {r:?}");
    }
}
