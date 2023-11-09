use crate::Float64;

use super::{combine_words, exp};

/// Exponential function, base 2
/// 
/// Calculate `e^x/2`. This is slightly more accurate than `0.5 * exp(x/2) * exp(x/2)`.
/// 
/// Only for `x >= log(DBL_MAX)`.
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub(crate) fn expo2(x: Float64) -> Float64 {
    /* k is such that k*ln2 has minimal relative error and x - kln2 > log(DBL_MIN) */
    const K: i32 = 2043;
    let kln2 = Float64::from_bits(0x40962066151add8b);

    /* note that k is odd and scale*scale overflows */
    let scale = combine_words(((0x3ff + K / 2) as u32) << 20, 0);
    /* exp(x - k ln2) * 2**(k-1) */
    exp(x - kln2) * scale * scale
}
