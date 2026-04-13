use crate::Number;
use std::fmt;

// ===========================================================================================
// ========================== Number impl ====================================================
// ===========================================================================================

impl Number {
    pub fn format(&self, formatting: Formatting) -> String {
        formatting.apply(self)
    }

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

// ===========================================================================================
// ========================== fmt impls ======================================================
// ===========================================================================================

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

// ===========================================================================================
// ========================== Formatting =====================================================
// ===========================================================================================

pub enum Formatting {
    /// How many digits to show after the decimal
    Decimal { keep_n_decimal_digits: usize },
    /// How many total digits to show. Symbols like `-` and `.` do not
    /// count towards digit count.
    /// If you have a decimal, `12345.678` and you format with `Digits { keep_n_digits: 6 }`
    /// the output will be `12345.6`.
    Digits { keep_n_digits: usize },
    /// Format as binary with separator and grouping.
    /// e.g., `1101011010101101010000011100101` using `Binary { separator: "x", grouping: 4 }` will
    /// output `1101x0110x1010x1101x0100x0001x1100x101`.
    /// e.g., `1101011010101101010000011100101` using Binary { separator: " ", grouping: 8 }` will
    /// output `11010110 10101101 01000001 1100101`.
    Binary { separator: String, grouping: usize },
}

impl Formatting {
    pub fn apply(&self, number: &Number) -> String {
        let num_str = number.to_string();

        match self {
            Formatting::Decimal {
                keep_n_decimal_digits,
            } => {
                if number.is_int() {
                    return num_str;
                }
                let (int_part, fract_part) = num_str.split_once('.').unwrap_or((&num_str, ""));
                let truncated_fract_part =
                    match fract_part.char_indices().nth(*keep_n_decimal_digits) {
                        None => return num_str, // fract part is shorter than n_decimals
                        Some((index, _)) => &fract_part[..index],
                    };
                format!("{int_part}.{truncated_fract_part}")
            }
            Formatting::Digits { keep_n_digits } => {
                let mut fmt_str = String::new();
                let mut iter = num_str.chars();
                let mut index = 0;
                while let Some(c) = iter.next()
                    && index < *keep_n_digits
                {
                    fmt_str.push(c);
                    if c != '-' && c != '.' {
                        index += 1;
                    }
                }
                fmt_str
            }
            Formatting::Binary {
                separator,
                grouping,
            } => {
                let bin_str = format!("{number:b}");
                let (int_part, fract_part) = bin_str.split_once('.').unwrap_or((&num_str, ""));
                let mut bin_int = Self::format_binary_str(int_part, separator, grouping);
                let bin_fract = Self::format_binary_str(fract_part, separator, grouping);
                if !bin_fract.is_empty() {
                    bin_int.push('.');
                    bin_int.push_str(&bin_fract);
                }
                bin_int
            }
        }
    }

    fn format_binary_str(bin_str: &str, separator: &str, grouping: &usize) -> String {
        if bin_str.is_empty() {
            return String::new();
        }
        let mut fmt_str = String::new();
        let mut iter = bin_str.chars().peekable();
        while iter.peek().is_some() {
            let mut curr_group_index: usize = 0;
            let mut curr_group = String::new();
            while curr_group_index < *grouping
                && let Some(cc) = iter.next()
            {
                curr_group.push(cc);
                if cc != '-' && cc != '.' {
                    curr_group_index += 1;
                }
            }
            if !curr_group.is_empty() {
                fmt_str.push_str(&curr_group);
            }
            if iter.peek().is_some() {
                fmt_str.push_str(separator);
            }
        }
        fmt_str
    }
}

// ===========================================================================================
// ========================== Tests ==========================================================
// ===========================================================================================

#[cfg(test)]
mod test {
    use super::*;
    use crate::NumberOrder;
    use rstest::*;

    #[test]
    fn formatting() {
        let n = "-12345.6789".parse::<Number>().unwrap();

        let decimals = n.format(Formatting::Decimal {
            keep_n_decimal_digits: 2,
        });
        let expected_decimals = "-12345.67".to_string();
        assert_eq!(
            decimals, expected_decimals,
            "expected decimals '{expected_decimals}' got decimals '{decimals}'"
        );

        let digits = n.format(Formatting::Digits { keep_n_digits: 7 });
        let expected_digits = "-12345.67".to_string();
        assert_eq!(
            digits, expected_digits,
            "expected digits '{expected_digits}' got digits '{digits}'"
        );

        let binary = n.format(Formatting::Binary {
            separator: " ".to_string(),
            grouping: 4,
        });
        let expected_binary = "-1100 0000 1110 01.1101 0100 0010 1".to_string();
        assert_eq!(
            binary, expected_binary,
            "expected binary '{expected_binary}' got binary '{binary}'"
        );
    }

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
