// For all `BigInt`/`BigDecimal`/`BigFloat` conversions to and from one another.

use super::Number;
use crate::number::CONSTS;
use astro_float::{
    BigFloat, Error as AstroErr, RoundingMode as AstroRoundingMode, Sign as AstroSign,
};
use bigdecimal::BigDecimal;
use num_bigint::{BigInt, Sign as BigIntSign};
use std::f64::consts::LOG2_10;

impl Number {
    pub fn to_bigfloat(&self) -> BigFloat {
        match self {
            Number::Int(i) => Self::bigint_to_bigfloat(i),
            Number::Decimal(d) => Self::bigdecimal_to_bigfloat(d),
        }
    }

    /// If your `BigInt` has `> 33_554_431` bits (`bi.bits()`) then this conversion
    /// will overflow, in which case we return NaN.
    fn bigint_to_bigfloat(bi: &BigInt) -> BigFloat {
        // Determine precision. Must be at least bi.bits() to avoid truncation.
        let p = std::cmp::max(bi.bits() as usize, 64);
        let (sign, limbs) = bi.to_u64_digits();
        let mut bf = BigFloat::from_word(0, p);

        for (i, &limb) in limbs.iter().enumerate() {
            if limb == 0 {
                continue;
            }

            let mut bf_word = BigFloat::from_word(limb, p);

            // Get the current exponent of this limb and shift it up by 64 bits per index.
            if let Some(e) = bf_word.exponent() {
                // This technically could overflow, e.g., `bf_limb.set_exponent(e + (i as i32 * 64));`
                if let Some(ev) = (i as i32).checked_mul(64).and_then(|cm| e.checked_add(cm)) {
                    bf_word.set_exponent(ev);
                } else {
                    return BigFloat::nan(Some(AstroErr::ExponentOverflow(AstroSign::Pos)));
                }
            }

            bf = bf.add(&bf_word, p, AstroRoundingMode::None);
        }

        if sign == BigIntSign::Minus {
            bf.inv_sign();
        }
        bf
    }

    fn bigdecimal_to_bigfloat(bd: &BigDecimal) -> BigFloat {
        let (bi, scale) = bd.as_bigint_and_scale(); // Conceptual: get raw parts
        let precision = ((scale as f64 * LOG2_10).ceil() as usize + 1).max(64);
        // 1. Convert integer mantissa to BigFloat (using our previous method)
        //let mantissa_bf = bigint_to_bigfloat_manual(bi, p);
        let mantissa_bf = Self::bigint_to_bigfloat(bi.as_ref());

        if scale == 0 {
            return mantissa_bf;
        }

        // 2. Efficiently compute 10^scale
        let ten = BigFloat::from_word(10, precision);

        let divisor = CONSTS.with(|cc| {
            ten.pow(
                &BigFloat::from_i64(scale, precision),
                precision,
                AstroRoundingMode::None,
                &mut cc.borrow_mut(),
            )
        });

        //let divisor = ten.pow(&BigFloat::from_i64(scale as i64, p), p, RoundingMode::None);

        // 3. Perform final division
        mantissa_bf.div(&divisor, precision, AstroRoundingMode::None)
        //.set_precision(precision, AstroRoundingMode::ToEven)
        //.expect("changed precision");
        //mantissa_bf
    }

    fn bigfloat_parts_to_bigint(
        &self,
        mantissa_limbs: &[u64],
        exponent: i32,
        precision: usize,
        sign: AstroSign,
    ) -> BigInt {
        let mut bi = BigInt::from(0i32);
        for (i, &limb) in mantissa_limbs.iter().enumerate() {
            bi += BigInt::from(limb) << (i * 64);
        }
        let p = precision as i32;
        if exponent >= p {
            bi <<= exponent - p;
        } else {
            bi >>= p - exponent;
        }
        if sign == AstroSign::Neg { -bi } else { bi }
    }
}
