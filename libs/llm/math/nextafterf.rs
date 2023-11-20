use crate::Float32;

/// Returns the next representable floating-point value following `x` in the direction of `y`.
#[export_name = "__llm_nextafterf"]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub extern "C" fn nextafterf(x: Float32, y: Float32) -> Float32 {
    if x.is_nan() || y.is_nan() {
        return x + y;
    }

    let mut ux_i = x.to_bits();
    let uy_i = y.to_bits();
    if ux_i == uy_i {
        return y;
    }

    let ax = ux_i & 0x7fff_ffff_u32;
    let ay = uy_i & 0x7fff_ffff_u32;
    if ax == 0 {
        if ay == 0 {
            return y;
        }
        ux_i = (uy_i & 0x8000_0000_u32) | 1;
    } else if ax > ay || ((ux_i ^ uy_i) & 0x8000_0000_u32) != 0 {
        ux_i -= 1;
    } else {
        ux_i += 1;
    }

    let e = ux_i & 0x7f80_0000_u32;
    // raise overflow if ux_f is infinite and x is finite
    if e == 0x7f80_0000_u32 {
        force_eval!(x + x);
    }
    let ux_f = Float32::from_bits(ux_i);
    // raise underflow if ux_f is subnormal or zero
    if e == 0 {
        force_eval!(x * x + ux_f * ux_f);
    }
    ux_f
}
