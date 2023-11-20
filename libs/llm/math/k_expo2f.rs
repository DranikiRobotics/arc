use crate::{Float32, Int};

use super::expf;

/* k is such that k*ln2 has minimal relative error and x - kln2 > log(FLT_MIN) */
const K: Int = 235;

/* expf(x)/2 for x >= log(FLT_MAX), slightly better than 0.5f*expf(x/2)*expf(x/2) */
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub(crate) fn k_expo2f(x: Float32) -> Float32 {
    let k_ln2 = Float32::from_bits(0x4322e3bc);
    /* note that k is odd and scale*scale overflows */
    let scale = Float32::from_bits(((0x7f + K / 2) as u32) << 23);
    /* exp(x - k ln2) * 2**(k-1) */
    expf(x - k_ln2) * scale * scale
}
