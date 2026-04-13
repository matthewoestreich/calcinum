use crate::Number;
use std::fmt;

impl Number {
    /// If variant is `Number::Decimal` we return the integer part is binary
    /// and the fractional part as binary, separated by a period.
    /// For example, if you have a `Number::Decimal(100.773)` this method
    /// returns : `"1100100.1100000101"`
    pub fn to_binary_str(&self) -> String {
        match self {
            Number::Int(big_int) => format!("{big_int:b}"),
            Number::Decimal(big_decimal) => {
                let s = big_decimal.to_string();
                let (lhs, rhs) = s.split_once('.').unwrap_or((&s, ""));
                let mut bin_str = Self::to_bin_str(lhs);
                if rhs.is_empty() {
                    bin_str
                } else {
                    bin_str.push('.');
                    bin_str.push_str(&Self::to_bin_str(rhs));
                    bin_str
                }
            }
        }
    }

    pub(crate) fn is_binary_str(s: &str) -> bool {
        s.starts_with("0b")
    }

    // Helper for `.to_binary_str`
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
            Number::Int(i) => write!(f, "{i}"),
            Number::Decimal(d) => write!(f, "{}", d.to_plain_string()),
        }
    }
}

impl fmt::Debug for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(i) => write!(f, "Number::Int({i})"),
            Self::Decimal(d) => write!(f, "Number::Decimal({})", d.to_plain_string()),
        }
    }
}

impl fmt::Binary for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_binary_str())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::NumberOrder;
    use rstest::*;

    #[rstest]
    #[case::fmt_display1("11.1", "11.1", NumberOrder::Decimal)]
    fn fmt_display(
        #[case] number: &str,
        #[case] expect_display: &str,
        #[case] expect_order: NumberOrder,
    ) {
        let x = number.parse::<Number>().unwrap();
        let r = x.order();
        assert_eq!(
            r, expect_order,
            "expected order '{expect_order:?}' got order '{r:?}'",
        );
        let r = format!("{x}");
        assert_eq!(
            r, expect_display,
            "expected display '{expect_display}' got display '{r}'"
        );
    }

    #[rstest]
    #[case::fmt_debug1("11.1", "Number::Decimal(11.1)")]
    fn fmt_debug(#[case] number: &str, #[case] expect_display: &str) {
        let x = number.parse::<Number>().unwrap();
        let r = format!("{x:?}");
        assert_eq!(
            r, expect_display,
            "expected debug '{expect_display}' got debug '{r}'"
        );
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
    fn fmt_binary_str(#[case] number: &str, #[case] expect: &str) {
        let n = number.parse::<Number>().unwrap();
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
}
