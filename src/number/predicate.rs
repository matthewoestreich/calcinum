use crate::{Number, number::digit::HexDigit};
use num_traits::{Signed, Zero};

impl Number {
    /// Returns `true` if `self` is equal to zero, and `false` if it is not.
    ///
    /// ```rust
    /// use calcinum::Number;
    ///
    /// let a = Number::from(0);
    /// assert!(a.is_zero() && a.is_int());
    ///
    /// let b = Number::from_f64_unchecked(0.0);
    /// assert!(b.is_zero() && b.is_decimal());
    /// ```
    pub fn is_zero(&self) -> bool {
        match self {
            Number::Int(i) => i.is_zero(),
            Number::Decimal(d) => d.is_zero(),
        }
    }

    /// Returns `true` if `self` is `Number::Int(_)` variant, `false` if not.
    ///
    /// ```rust
    /// use calcinum::Number;
    ///
    /// let n = Number::from(0);
    /// assert!(n.is_int());
    /// ```
    pub fn is_int(&self) -> bool {
        matches!(self, Number::Int(_))
    }

    /// Returns `true` if `self` is `Number::Decimal(_)` variant, `false` if not.
    ///
    /// ```rust
    /// use calcinum::Number;
    ///
    /// let n = Number::from_f64_unchecked(0.0);
    /// assert!(n.is_decimal());
    /// ```
    pub fn is_decimal(&self) -> bool {
        matches!(self, Number::Decimal(_))
    }

    /// Returns `true` if the number is negative, `false` if not.
    ///
    /// ```rust
    /// use calcinum::Number;
    ///
    /// let a = Number::from(1);
    /// assert!(!a.is_negative());
    ///
    /// let b = Number::from(-1);
    /// assert!(b.is_negative());
    ///
    /// // `0` is neither positive nor negative.
    /// let c = Number::from(0);
    /// assert!(!c.is_negative() && !c.is_positive());
    /// // Use `.is_zero()` method instead.
    /// assert!(c.is_zero());
    ///
    /// // `-0` is neither positive nor negative.
    /// let d = Number::from(-0);
    /// assert!(!d.is_negative() && !d.is_positive());
    /// // Use `.is_zero()` method instead.
    /// assert!(d.is_zero());
    /// ```
    pub fn is_negative(&self) -> bool {
        match self {
            Number::Int(i) => i.is_negative(),
            Number::Decimal(d) => d.is_negative(),
        }
    }

    /// Returns `true` if the number is positive, `false` if not.
    ///
    /// ```rust
    /// use calcinum::Number;
    ///
    /// let a = Number::from(1);
    /// assert!(a.is_positive());
    ///
    /// let b = Number::from(-1);
    /// assert!(!b.is_positive());
    ///
    /// // `0` is neither positive nor negative.
    /// let c = Number::from(0);
    /// assert!(!c.is_negative() && !c.is_positive());
    /// // Use `.is_zero()` method instead.
    /// assert!(c.is_zero());
    ///
    /// // `-0` is neither positive nor negative.
    /// let d = Number::from(-0);
    /// assert!(!d.is_negative() && !d.is_positive());
    /// // Use `.is_zero()` method instead.
    /// assert!(d.is_zero());
    /// ```
    pub fn is_positive(&self) -> bool {
        match self {
            Number::Int(i) => i.is_positive(),
            Number::Decimal(d) => d.is_positive(),
        }
    }
}

/// If `validate_prefix` is true, we expect a binary string to start with
/// `"0b"` or `"-0b"` for negative binary strings.
/// A binary string can contain:
/// - Digits `0` or `1`.
/// - A single negative sign, e.g., `-`, required to be at the start of the string
/// - A decimal, e.g., `.` to denote a fractional number in binary form.
pub(crate) fn is_binary_str(s: &str, validate_prefix: bool) -> bool {
    if s.is_empty() {
        return false;
    }
    if validate_prefix && (!s.starts_with("0b") && !s.starts_with("-0b")) {
        return false;
    }

    let s = s.strip_prefix('-').unwrap_or(s);
    let s = if validate_prefix {
        s.strip_prefix("0b").unwrap_or(s)
    } else {
        s
    };

    let mut seen_decimal = false;

    for c in s.chars() {
        match c {
            // We should not see any other '-' signs.
            '-' => return false,
            '.' if !seen_decimal => seen_decimal = true,
            c if c == '0' || c == '1' => {}
            _ => return false,
        }
    }

    true
}

/// If `validate_prefix` is true, we expect a hexadecimal string to start with `"0x"` or `"-0x"`.
/// An empty string will return `false`.
/// A hexadecimal string can contain (in any order):
/// - Digits `0` - `9`.
/// - Characters (case insensitive) `A`, `B`, `C`, `D`, `E`, `F`.
/// - A single negative sign, e.g., `-`, required to be at the start of the string, after the `"0b"` prefix.
/// - A decimal, e.g., `.` to denote a fractional number in binary form.
pub(crate) fn is_hexadecimal_str(s: &str, validate_prefix: bool) -> bool {
    if s.is_empty() {
        return false;
    }
    if validate_prefix && (!s.starts_with("-0x") && !s.starts_with("0x")) {
        return false;
    }

    let s = s.strip_prefix('-').unwrap_or(s);
    let s = if validate_prefix {
        s.strip_prefix("0x").unwrap_or(s)
    } else {
        s
    };

    let mut seen_decimal = false;

    for c in s.chars() {
        match c {
            // We should not see any other '-' signs.
            '-' => return false,
            '.' if !seen_decimal => seen_decimal = true,
            c if HexDigit::try_from(c).is_ok() => {}
            _ => return false,
        }
    }

    true
}

/// Octal string must start with `0o` or `-0o` for negative octal strings.
/// A valid octal string on contains characters "0" - "7".
pub(crate) fn is_octal_str(s: &str, validate_prefix: bool) -> bool {
    if s.is_empty() {
        return false;
    }
    if validate_prefix && (!s.starts_with("-0o") && !s.starts_with("0o")) {
        return false;
    }

    let s = s.strip_prefix('-').unwrap_or(s);
    let s = if validate_prefix {
        s.strip_prefix("0o").unwrap_or(s)
    } else {
        s
    };

    let mut seen_decimal = false;

    for c in s.chars() {
        match c {
            // Already checked front.
            '-' => return false,
            '.' if !seen_decimal => seen_decimal = true,
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' => {}
            _ => return false,
        }
    }

    true
}

/// Checks to see if a string is considered a decimal.
/// An empty decimal string returns `false`.
/// We expect a decimal string to contain only:
/// - Digits `0`-`9`.
/// - A single negative sign, e.g., `-`, required to be at the start of the string.
/// - A decimal, e.g., `.` to denote a decimal with a fractional part.
pub(crate) fn is_decimal_str(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    let s = s.strip_prefix('-').unwrap_or(s);
    let mut seen_decimal = false;

    for c in s.chars() {
        match c {
            // We should not see any other '-' signs.
            '-' => return false,
            '.' if !seen_decimal => seen_decimal = true,
            c if c.is_ascii_digit() => {}
            _ => return false,
        }
    }

    true
}
