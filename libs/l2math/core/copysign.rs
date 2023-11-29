use crate::Float64;

/// Sign of Y, magnitude of X
///
/// Constructs a number with the magnitude (absolute value) of its
/// first argument, `x`, and the sign of its second argument, `y`.
#[export_name = "__l2math_copysign"]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub extern "C" fn copysign(x: Float64, y: Float64) -> Float64 {
    let mut ux = x.to_bits();
    let uy = y.to_bits();
    ux &= (!0) >> 1;
    ux |= uy & (1 << 63);
    Float64::from_bits(ux)
}
