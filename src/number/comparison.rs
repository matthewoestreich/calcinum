use crate::Number;
use num_traits::Zero;

impl Number {
    /// Returns `true` if `self` is equal to zero, and `false` if it is not.
    ///
    /// ```rust
    /// use calcinum::Number;
    ///
    /// let n = Number::from(0);
    /// let is_int_variant = matches!(n, Number::Int(_));
    /// assert!(is_int_variant);
    /// assert!(n.is_zero());
    ///
    /// let n = Number::from_f64_unchecked(0.0);
    /// let is_decimal_variant = matches!(n, Number::Decimal(_));
    /// assert!(is_decimal_variant);
    /// assert!(n.is_zero());
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
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(l), Self::Int(r)) => l == r,
            (Self::Decimal(l), Self::Decimal(r)) => l == r,
            _ => false,
        }
    }
}

impl Eq for Number {}
