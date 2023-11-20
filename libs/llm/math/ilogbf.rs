use crate::{Float32, Int};

const FP_ILOGBNAN: Int = -1 - 0x7fffffff;
const FP_ILOGB0: Int = FP_ILOGBNAN;

/// Get exponent of floating point value
#[export_name = "__llm_ilogbf"]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
#[allow(clippy::zero_divided_by_zero)]
pub fn ilogbf(x: Float32) -> Int {
    let mut i = x.to_bits();
    let e = ((i >> 23) & 0xff) as Int;

    if e == 0 {
        i <<= 9;
        if i == 0 {
            force_eval!(0.0 / 0.0);
            return FP_ILOGB0;
        }
        /* subnormal x */
        let mut e = -0x7f;
        while (i >> 31) == 0 {
            e -= 1;
            i <<= 1;
        }
        e
    } else if e == 0xff {
        force_eval!(0.0 / 0.0);
        if (i << 9) != 0 {
            FP_ILOGBNAN
        } else {
            Int::max_value()
        }
    } else {
        e - 0x7f
    }
}
