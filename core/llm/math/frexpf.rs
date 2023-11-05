use crate::Float32;

/// Breaks the number into a normalized fraction and a base-2 exponent
/// 
/// Satisfying:
/// 
/// * `x = f * 2^exp`
/// * `0.5 <= abs(f) < 1.0`
pub fn frexpf(x: Float32) -> (Float32, i32) {
    let mut y = x.to_bits();
    let ee: i32 = ((y >> 23) & 0xff) as i32;

    if ee == 0 {
        if x != 0.0 {
            let x1p64 = Float32::from_bits(0x5f800000);
            let (x, e) = frexpf(x * x1p64);
            return (x, e - 64);
        } else {
            return (x, 0);
        }
    } else if ee == 0xff {
        return (x, 0);
    }

    let e = ee - 0x7e;
    y &= 0x807fffff;
    y |= 0x3f000000;
    (Float32::from_bits(y), e)
}
