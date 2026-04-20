use crate::Number;
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
