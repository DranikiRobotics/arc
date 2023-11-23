use crate::{Float64, Float32};

const NEG_1_SHIFT_1R: u32 = 0x7FFFFFFF;

use super::sqrtf;

/// Calculate the length of the hypotenuse of a right-angle triangle given legs of length x and y.
#[export_name = "__l2math_hypotf"]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub extern "C" fn hypotf(mut x: Float32, mut y: Float32) -> Float32 {
    let x1p90 = Float32::from_bits(0x6c800000); // 0x1p90f === 2 ^ 90
    let x1p_90 = Float32::from_bits(0x12800000); // 0x1p-90f === 2 ^ -90

    let mut uxi = x.to_bits();
    let mut uyi = y.to_bits();
    let uti;
    let mut z: Float32;

    uxi &= NEG_1_SHIFT_1R;
    uyi &= NEG_1_SHIFT_1R;
    if uxi < uyi {
        uti = uxi;
        uxi = uyi;
        uyi = uti;
    }

    x = Float32::from_bits(uxi);
    y = Float32::from_bits(uyi);
    if uyi == 0xff << 23 {
        return y;
    }
    if uxi >= 0xff << 23 || uyi == 0 || uxi - uyi >= 25 << 23 {
        return x + y;
    }

    z = 1.;
    if uxi >= (0x7f + 60) << 23 {
        z = x1p90;
        x *= x1p_90;
        y *= x1p_90;
    } else if uyi < (0x7f - 60) << 23 {
        z = x1p_90;
        x *= x1p90;
        y *= x1p90;
    }
    z * sqrtf((x as Float64 * x as Float64 + y as Float64 * y as Float64) as Float32)
}
