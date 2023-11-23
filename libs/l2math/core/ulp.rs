use crate::Float64;

const MANTISSA_BITS: u64 = 52;
const EXPONENT_BITS: u64 = 11;
const POS_ZERO_BITS: u64 = 0x0000000000000000; 
const NEG_ZERO_BITS: u64 = 0x8000000000000000;

/// Returns the value of the least significant bit of the given floating point number.
/// 
/// Note: this function can technically be `const` but we need to wait for:
/// - [`const_fn_floating_point_arithmetic`](https://github.com/rust-lang/rust/issues/57241)
/// - [`const_float_classify`](https://github.com/rust-lang/rust/issues/72505)
/// - [`const_fn_floating_point_arithmetic`](https://github.com/rust-lang/rust/issues/57241)
#[inline]
#[allow(unused_variables)]
#[export_name = "__l2math_ulp"]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
#[must_use = "This function returns the least significant bit of the argument."]
pub extern "C" fn ulp(x: Float64) -> Float64 {
    // SAFETY: x is a normal number, so it is finite and not zero.
    if x.is_nan() {
        return x;
    }
    if x.is_infinite() {
        return Float64::INFINITY;
    }
    if x == Float64::MAX || x == -Float64::MAX ||
       x == Float64::MIN || x == -Float64::MIN ||
       x == Float64::MIN_POSITIVE || x == -Float64::MIN_POSITIVE {
        return x;
    }

    // Last sanity check
    let bits = x.to_bits();

    if bits == POS_ZERO_BITS {
        return Float64::MIN_POSITIVE;
    }
    if bits == NEG_ZERO_BITS {
        return -Float64::MIN_POSITIVE;
    }

    // Actual code
    let mant_mask = (1 << MANTISSA_BITS) - 1;
    let mantissa = bits & mant_mask;
    let exp_mask = (1 << EXPONENT_BITS) - 1;
    let exponent = (bits >> MANTISSA_BITS) & exp_mask;
    if exponent == 0 {
        let result = (exponent << MANTISSA_BITS) | 1;
        return Float64::from_bits(result);
    };
    let mut new_exponent = exponent - MANTISSA_BITS;
    let new_mantissa: u64;
    if new_exponent > 0 {
        new_mantissa = 0;
    } else {
        new_mantissa = 1 << -(new_exponent as i64 - 1);
        new_exponent = 0;
    };
    Float64::from_bits((new_exponent << MANTISSA_BITS) | new_mantissa)
}

#[test]
fn zero_ulp() {
    assert_eq!(ulp(0.0), Float64::MIN_POSITIVE);
    assert_eq!(ulp(-0.0), -Float64::MIN_POSITIVE);
}

#[test]
fn one_ulp() {
    assert_eq!(ulp(1.0), Float64::EPSILON);
    assert_eq!(ulp(-1.0), Float64::EPSILON);
}

#[test]
fn two_ulp() {
    assert_eq!(ulp(2.0), Float64::EPSILON * 2.0);
    assert_eq!(ulp(-2.0), Float64::EPSILON * 2.0);
}

#[test]
fn half_ulp() {
    assert_eq!(ulp(0.5), Float64::EPSILON / 2.0);
    assert_eq!(ulp(-0.5), Float64::EPSILON / 2.0);
}

#[test]
fn max_value_ulp() {
    assert_eq!(ulp(Float64::MAX), Float64::MAX);
    assert_eq!(ulp(-Float64::MAX), -Float64::MAX);
}

#[test]
fn min_value_ulp() {
    assert_eq!(ulp(Float64::MIN), Float64::MIN);
    assert_eq!(ulp(-Float64::MIN), -Float64::MIN);
}

#[test]
fn infinity_ulp() {
    assert_eq!(ulp(Float64::INFINITY), Float64::INFINITY);
    assert_eq!(ulp(-Float64::INFINITY), Float64::INFINITY);
}

#[test]
fn nan_ulp() {
    assert!(ulp(Float64::NAN).is_nan());
    assert!(ulp(-Float64::NAN).is_nan());
}

#[test]
fn subnormal_ulp() {
    assert_eq!(ulp(Float64::MIN_POSITIVE), Float64::MIN_POSITIVE);
    assert_eq!(ulp(-Float64::MIN_POSITIVE), -Float64::MIN_POSITIVE);
}
