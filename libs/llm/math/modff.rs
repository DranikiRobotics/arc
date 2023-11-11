use crate::Float32;

/// Breaks the given number into an integral and a fractional part.
pub fn modff(x: Float32) -> (Float32, Float32) {
    let rv2: Float32;
    let mut u: u32 = x.to_bits();
    let e = ((u >> 23 & 0xff) as i32) - 0x7f;

    /* no fractional part */
    if e >= 23 {
        rv2 = x;
        if e == 0x80 && (u << 9) != 0 {
            /* nan */
            return (x, rv2);
        }
        u &= 0x80000000;
        return (Float32::from_bits(u), rv2);
    }
    /* no integral part */
    if e < 0 {
        u &= 0x80000000;
        rv2 = Float32::from_bits(u);
        return (x, rv2);
    }

    let mask: u32 = 0x007fffff >> e;
    if (u & mask) == 0 {
        rv2 = x;
        u &= 0x80000000;
        return (Float32::from_bits(u), rv2);
    }
    u &= !mask;
    rv2 = Float32::from_bits(u);
    return (x - rv2, rv2);
}
