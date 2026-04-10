/// Expects `$lhs` to be `&Number`
/// Expects `$rhs` to be `&Number`
/// Expects `$op` to be an operator (+, -, /, *, %)
macro_rules! match_arithmetic {
    ($lhs:expr, $rhs:expr, $op:tt) => {
        match ($lhs, $rhs) {
            (Number::Int(x), Number::Int(y)) => Number::Int(x $op y),
            (Number::Decimal(x), Number::Decimal(y)) => Number::Decimal(x $op y),
            (Number::Int(x), Number::Decimal(y)) => {
                let x = BigDecimal::from_bigint(x.clone(), 0);
                Number::Decimal(x $op y)
            }
            (Number::Decimal(x), Number::Int(y)) => {
                let y = BigDecimal::from_bigint(y.clone(), 0);
                Number::Decimal(x $op y)
            }
        }
    };
}

/// Expects `$lhs` to be `&mut Number`
/// Expects `$rhs` to be `&Number`
/// Expects `$op` to be an operator (+, -, /, *, %)
macro_rules! match_arithmetic_assign {
    ($lhs:expr, $rhs:expr, $op:tt) => {
        *$lhs = match (&$lhs, $rhs) {
            (Number::Int(x), Number::Int(y)) => Number::Int(x $op y),
            (Number::Decimal(x), Number::Decimal(y)) => Number::Decimal(x $op y),
            (Number::Decimal(x), Number::Int(y)) => {
                let y = BigDecimal::from_bigint(y.clone(), 0);
                Number::Decimal(x $op y)
            }
            (Number::Int(_), Number::Decimal(_)) => {
                $lhs.promote();
                &*$lhs $op $rhs
            }
        }
    };
}

/// Expects `$lhs` to be `&Number`
/// Expects `$rhs` to be `&Number`
/// Expects `$op` to be a bitwise operator (&, |, ^)
/// IMPORTANT : we can only perform bitwise operations on Number::Int.
/// IMPORTANT : If either side is Number::Decimal we conver the Decimal
/// into an integer before calling the bitwise operation, which may result
/// in unexpected calculations!
macro_rules! match_bitwise {
    ($lhs:expr, $rhs:expr, $op:tt) => {
        match ($lhs, $rhs) {
            (Number::Int(x), Number::Int(y)) => Number::Int(x $op y),
            (Number::Decimal(x), Number::Decimal(y)) => {
                let x = x.to_bigint().expect("BigInt");
                let y = y.to_bigint().expect("BigInt");
                Number::Int(x $op y)
            }
            (Number::Int(x), Number::Decimal(y)) => {
                let y = y.to_bigint().expect("BigInt");
                Number::Int(x $op y)
            }
            (Number::Decimal(x), Number::Int(y)) => {
                let x = x.to_bigint().expect("BigInt");
                Number::Int(x $op y)
            }
        }
    };
}

/// Expects `$lhs` to be `&mut Number`
/// Expects `$rhs` to be `&Number`
/// Expects `$op` to be a bitwise operator (&, |, ^)
/// IMPORTANT : we can only perform bitwise operations on Number::Int.
/// IMPORTANT : If either side is Number::Decimal we convert the Decimal
/// into an integer before calling the bitwise operation, which may result
/// in unexpected calculations!
macro_rules! match_bitwise_assign {
    ($lhs:expr, $rhs:expr, $op:tt) => {
        *$lhs = match (&$lhs, $rhs) {
            (Number::Int(x), Number::Int(y)) => Number::Int(x $op y),
            (Number::Int(x), Number::Decimal(y)) => {
                let y = y.to_bigint().expect("BigInt");
                Number::Int(x $op y)
            }
            _ => {
                $lhs.demote();
                &*$lhs $op $rhs
            }
        }
    };
}

/// Expects `$lhs` to be `&Number`
/// Expects `$rhs` to be `&Number`
/// Expects `$op` to be a bitwise shift (<< | >>)
/// IMPORTANT : If either side is `Number::Deimal` variant, we demote it to `Number::Int`.
/// IMPORTANT : We can only right shift by numbers that fit within an i128! If your right
/// hand side does not it within an i128 it will be satured, which may result in data loss!
macro_rules! match_shift {
    ($lhs:expr, $rhs:expr, $op:tt) => {
        match ($lhs, $rhs) {
            (Number::Int(x), Number::Int(_)) => {
                let y = $rhs.to_i128_saturating();
                Number::from(x $op y)
            }
            (Number::Decimal(x), Number::Decimal(_)) => {
                let x = x.to_bigint().expect("BigInt");
                let y = $rhs.to_i128_saturating();
                Number::from(x $op y)
            }
            (Number::Int(x), Number::Decimal(_)) => {
                let y = $rhs.to_i128_saturating();
                Number::from(x $op y)
            }
            (Number::Decimal(x), Number::Int(_)) => {
                let x = x.to_bigint().expect("BigInt");
                let y = $rhs.to_i128_saturating();
                Number::from(x $op y)
            }
        }
    };
}

/// Expects `$lhs` to be `&mut Number`
/// Expects `$rhs` to be `&Number`
/// Expects `$op` to be a bitwise shift (<< | >>)
/// IMPORTANT : If $lhs is `Number::Deimal` variant, we demote it to `Number::Int`.
/// IMPORTANT : We can only right shift by numbers that fit within an i128! If your right
/// hand side does not it within an i128 it will be satured, which may result in data loss!
macro_rules! match_shift_assign {
    ($lhs:expr, $rhs:expr, $op:tt) => {
        *$lhs = match (&$lhs, $rhs) {
            (Number::Int(x), Number::Int(_)) => {
                let y = $rhs.to_i128_saturating();
                Number::from(x $op y)
            }
            (Number::Int(x), Number::Decimal(_)) => {
                let y = $rhs.to_i128_saturating();
                Number::from(x $op y)
            }
            _ => {
                $lhs.demote();
                &*$lhs $op $rhs
            }
        }
    };
}
