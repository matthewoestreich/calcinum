mod from;
mod to;

pub(crate) use to::*;

use crate::{Number, ToNumber};
use bigdecimal::BigDecimal;
use num_bigint::BigInt;

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
// ========================== Tests ==========================================================
// ===========================================================================================

#[cfg(test)]
mod test {
    use crate::number::ToNumber;
    use crate::number::conversion;
    use crate::*;
    use rstest::*;

    #[test]
    fn round_trip_binary_conversion() {
        let i = 123.to_number(); // Number::Int(123)
        let bs = format!("{i:b}"); // "1111011"
        // Parse binary string back into `Number` - needs "0b" prefix.
        let s = format!("0b{bs}");
        let n = s.parse::<Number>().unwrap(); // Number::Int(123)
        assert_eq!(i, n);

        let i = 382.619.to_number(); // Number::Decimal(382.619)
        let bs = format!("{i:b}"); // "1111011"
        // Parse binary string back into `Number` - needs "0b" prefix.
        let s = format!("0b{bs}");
        let n = s.parse::<Number>().unwrap(); // Number::Decimal(382.619)
        assert_eq!(i, n);
    }

    #[rstest]
    #[case::b64_roundtrip("-2345.1235", "LTIzNDUuMTIzNQ==")]
    #[case::b64_roundtrip("43543.322938403", "NDM1NDMuMzIyOTM4NDAz")]
    #[case::b64_roundtrip("4352439852433149", "NDM1MjQzOTg1MjQzMzE0OQ==")]
    #[case::b64_roundtrip("-000000000.0000000000", "LTAwMDAwMDAwMC4wMDAwMDAwMDAw")]
    fn base64_encode_decode(#[case] s: &str, #[case] expect: &str) {
        let encoded = conversion::base64_encode(s);
        assert_eq!(
            encoded, expect,
            "expected encoded '{expect}' got encoded '{encoded}'"
        );
        let decoded = conversion::from::base64_decode(&encoded);
        assert_eq!(
            decoded,
            s.to_string(),
            "expected decoded = '{s}' got decoded '{decoded}'"
        );
    }
}
