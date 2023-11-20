use crate::{Float64, Int};

const FP_ILOGBNAN: Int = -1 - 0x7fffffff;
const FP_ILOGB0: Int = FP_ILOGBNAN;

/// Get exponent of floating point value
#[export_name = "__llm_ilogb"]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
#[allow(clippy::zero_divided_by_zero)]
pub fn ilogb(x: Float64) -> Int {
    let mut i: u64 = x.to_bits();
    let e = ((i >> 52) & 0x7ff) as Int;

    if e == 0 {
        i <<= 12;
        if i == 0 {
            force_eval!(0.0 / 0.0);
            return FP_ILOGB0;
        }
        /* subnormal x */
        let mut e = -0x3ff;
        while (i >> 63) == 0 {
            e -= 1;
            i <<= 1;
        }
        e
    } else if e == 0x7ff {
        force_eval!(0.0 / 0.0);
        if (i << 12) != 0 {
            FP_ILOGBNAN
        } else {
            Int::max_value()
        }
    } else {
        e - 0x3ff
    }
}
