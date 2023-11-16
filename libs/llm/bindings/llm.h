#ifndef _MATH_H
#define _MATH_H

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/// High precision floating point number
typedef double Float64;
/// Low precision floating point number
typedef float Float32;
/// High precision floating point number representing an angle
typedef double Radian64;
/// Low precision floating point number representing an angle
typedef float Radian32;
/// C alternative to rust's i32
typedef int Int;

/// A structure representing a tuple of a `Float64` and an `Int`
typedef struct Tuple_Float64_Int {
    Float64 f;
    Int i;
} Tuple_Float64_Int;

/// A structure representing a tuple of a `Float32` and an `Int`
typedef struct Tuple_Float32_Int {
    Float32 f;
    Int i;
} Tuple_Float32_Int;

/// A structure representing a tuple of two `Float64`s
typedef struct Tuple_Float64_Float64 {
    Float64 f1;
    Float64 f2;
} Tuple_Float64_Float64;

/// A structure representing a tuple of two `Float32`s
typedef struct Tuple_Float32_Float32 {
    Float32 f1;
    Float32 f2;
} Tuple_Float32_Float32;

/// Arccosine
Radian64 __llm_acos(Float64 x);
/// Arccosine (Float32)
Radian32 __llm_acosf(Float32 x);
/// Inverse hyperbolic cosine
Float64 __llm_acosh(Float64 x);
/// Inverse hyperbolic cosine (Float32)
Float32 __llm_acoshf(Float32 x);
/// Arcsine
Radian64 __llm_asin(Float64 x);
/// Arcsine (Float32)
Radian32 __llm_asinf(Float32 x);
/// Inverse hyperbolic sine
Float64 __llm_asinh(Float64 x);
/// Inverse hyperbolic sine (Float32)
Float32 __llm_asinhf(Float32 x);
/// Arctangent
Radian64 __llm_atan(Float64 x);
/// Arctangent of y/x
Radian64 __llm_atan2(Float64 y, Float64 x);
/// Arctangent of y/x (Float32)
Radian32 __llm_atan2f(Float32 y, Float32 x);
/// Arctangent (Float32)
Radian32 __llm_atanf(Float32 x);
/// Inverse hyperbolic tangent
Float64 __llm_atanh(Float64 x);
/// Inverse hyperbolic tangent (Float32)
Float32 __llm_atanhf(Float32 x);
/// Computes the cube root of the argument.
Float64 __llm_cbrt(Float64 x);
/// Cube root (Float32)
Float32 __llm_cbrtf(Float32 x);
/// Ceil
Float64 __llm_ceil(Float64 x);
/// Ceil (Float32)
Float32 __llm_ceilf(Float32 x);
/// Sign of Y, magnitude of X
Float64 __llm_copysign(Float64 x, Float64 y);
/// Sign of Y, magnitude of X (Float32)
Float32 __llm_copysignf(Float32 x, Float32 y);
/// Cosine
Float64 __llm_cos(Radian64 x);
/// Cosine (Float32)
Float32 __llm_cosf(Radian32 x);
/// Hyperbolic cosine
Float64 __llm_cosh(Float64 x);
/// Hyperbolic cosine (Float32)
Float32 __llm_coshf(Float32 x);
/// Computes the exponential function of x.
Float64 __llm_exp(Float64 x);
/// Computes the exponential function of x (Float32).
Float32 __llm_expf(Float32 x);
/// Computes the exponential minus one function of x.
Float64 __llm_expm1(Float64 x);
/// Computes the exponential minus one function of x (Float32).
Float32 __llm_expm1f(Float32 x);
/// Absolute value
Float64 __llm_fabs(Float64 x);
/// Absolute value (Float32)
Float32 __llm_fabsf(Float32 x);
/// Factorial
Float64 __llm_factorial(Float64 x);
/// Factorial (Float32)
Float32 __llm_factorialf(Float32 x);
/// Floor
Float64 __llm_floor(Float64 x);
/// Floor (Float32)
Float32 __llm_floorf(Float32 x);
/// Computes the fused multiply-add of x, y and z.
Float64 __llm_fma(Float64 x, Float64 y, Float64 z);
/// Computes the fused multiply-add of x, y and z (Float32).
Float32 __llm_fmaf(Float32 x, Float32 y, Float32 z);
/// Returns the maximum of the two arguments, signaling NaNs if either argument is a signaling NaN.
Float64 __llm_fmax(Float64 x, Float64 y);
/// Returns the maximum of the two arguments, signaling NaNs if either argument is a signaling NaN (Float32).
Float32 __llm_fmaxf(Float32 x, Float32 y);
/// Returns the minimum of the two arguments, signaling NaNs if either argument is a signaling NaN.
Float64 __llm_fmin(Float64 x, Float64 y);
/// Returns the minimum of the two arguments, signaling NaNs if either argument is a signaling NaN (Float32).
Float32 __llm_fminf(Float32 x, Float32 y);
/// Computes the floating-point remainder of dividing x by y.
Float64 __llm_fmod(Float64 x, Float64 y);
/// Computes the floating-point remainder of dividing x by y (Float32).
Float32 __llm_fmodf(Float32 x, Float32 y);
/// Breaks the number into a normalized fraction and a base-2 exponent
Tuple_Float64_Int __llm_frexp(Float64 x);
/// Breaks the number into a normalized fraction and a base-2 exponent (Float32)
Tuple_Float32_Int __llm_frexpf(Float32 x);
/// Calculate the length of the hypotenuse of a right-angle triangle given legs of length x and y.
Float64 __llm_hypot(Float64 x, Float64 y);
/// Calculate the length of the hypotenuse of a right-angle triangle given legs of length x and y (Float32).
Float32 __llm_hypotf(Float32 x, Float32 y);
/// Get exponent of floating point value
Int __llm_ilogb(Float64 x);
/// Get exponent of floating point value (Float32)
Int __llm_ilogbf(Float32 x);
/// Bessel function of the first kind of order zero
Float64 __llm_j0(Float64 x);
/// Bessel function of the first kind of order zero (Float32)
Float32 __llm_j0f(Float32 x);
/// Bessel function of the first kind of order one
Float64 __llm_j1(Float64 x);
/// Bessel function of the first kind of order one (Float32)
Float32 __llm_j1f(Float32 x);
/// Bessel function of the first kind of order n
Float64 __llm_jn(Int n, Float64 x);
/// Bessel function of the first kind of order n (Float32)
Float32 __llm_jnf(Int n, Float32 x);
/// Returns x * 2^n.
Float64 __llm_ldexp(Float64 x, Int n);
/// Returns x * 2^n (Float32).
Float32 __llm_ldexpf(Float32 x, Int n);
/// Natural logarithm of gamma function
Float64 __llm_lgamma(Float64 x);
/// Natural logarithm of gamma function (Float32)
Float32 __llm_lgammaf(Float32 x);
/// Natural logarithm of gamma function
Tuple_Float64_Int __llm_lgamma_r(Float64 x);
/// Natural logarithm of gamma function (Float32)
Tuple_Float32_Int __llm_lgammaf_r(Float32 x);
/// Return the natural logarithm of x.
Float64 __llm_ln(Float64 x);
/// Return the natural logarithm of x (Float32).
Float32 __llm_lnf(Float32 x);
/// Return logarithm of x.
Float64 __llm_log(Float64 x);
/// Return logarithm of x (Float32).
Float32 __llm_logf(Float32 x);
/// Return logarithm of x to base 10.
Float64 __llm_log10(Float64 x);
/// Return logarithm of x to base 10 (Float32).
Float32 __llm_log10f(Float32 x);
/// Return logarithm of x to base 2.
Float64 __llm_log2(Float64 x);
/// Return logarithm of x to base 2 (Float32).
Float32 __llm_log2f(Float32 x);
/// Return the natural logarithm of one plus x.
Float64 __llm_log1p(Float64 x);
/// Return the natural logarithm of one plus x (Float32).
Float32 __llm_log1pf(Float32 x);
/// Breaks the given number into an integral and a fractional part.
Tuple_Float64_Float64 __llm_modf(Float64 x);
/// Breaks the given number into an integral and a fractional part (Float32).
Tuple_Float32_Float32 __llm_modff(Float32 x);
/// Returns the next representable floating-point value following x in the direction of y.
Float64 __llm_nextafter(Float64 x, Float64 y);
/// Returns the next representable floating-point value following x in the direction of y. (Float32)
Float32 __llm_nextafterf(Float32 x, Float32 y);
/// Returns x raised to the power y.
Float64 __llm_pow(Float64 x, Float64 y);
/// Returns x raised to the power y. (Float32)
Float32 __llm_powf(Float32 x, Float32 y);
/// Returns the remainder of x/y.
Float64 __llm_remainder(Float64 x, Float64 y);
/// Returns the remainder of x/y. (Float32)
Float32 __llm_remainderf(Float32 x, Float32 y);
/// Return the remainder and part of the quotient of x and y.
Tuple_Float64_Int __llm_remquo(Float64 x, Float64 y);
/// Return the remainder and part of the quotient of x and y. (Float32)
Tuple_Float32_Int __llm_remquof(Float32 x, Float32 y);
/// Round to nearest integer, rounding halfway cases away from zero.
Float64 __llm_rint(Float64 x);
/// Round to nearest integer, rounding halfway cases away from zero. (Float32)
Float32 __llm_rintf(Float32 x);
/// Rounds x to the nearest integer in the direction of the current rounding mode.
Float64 __llm_round(Float64 x);
/// Rounds x to the nearest integer in the direction of the current rounding mode. (Float32)
Float32 __llm_roundf(Float32 x);
/// Returns x * 2^n
Float64 __llm_scalbn(Float64 x, Int n);
/// Returns x * 2^n (Float32)
Float32 __llm_scalbnf(Float32 x, Int n);
/// Returns the sine function of x.
Float64 __llm_sin(Radian64 x);
/// Simultaneously computes the sine and cosine of the argument x.
Tuple_Float64_Float64 __llm_sincos(Radian64 x);
/// Simultaneously computes the sine and cosine of the argument x. (Float32)
Tuple_Float32_Float32 __llm_sincosf(Radian32 x);
/// Returns the sine of the argument x.
Float64 __llm_sinf(Float64 x);
/// Returns the hyperbolic sine of x.
Float64 __llm_sinh(Float64 x);
/// Returns the hyperbolic sine of x. (Float32)
Float32 __llm_sinhf(Float32 x);
/// Returns the square root of x.
Float64 __llm_sqrt(Float64 x);
/// Returns the square root of x. (Float32)
Float32 __llm_sqrtf(Float32 x);
/// Returns tangent of x.
Float64 __llm_tan(Radian64 x);
/// Returns tangent of x. (Float32)
Float32 __llm_tanf(Radian32 x);
/// Returns the hyperbolic tangent of x.
Float64 __llm_tanh(Float64 x);
/// Returns the hyperbolic tangent of x. (Float32)
Float32 __llm_tanhf(Float32 x);
/// Returns the gamma function of x.
Float64 __llm_tgamma(Float64 x);
/// Returns the gamma function of x. (Float32)
Float32 __llm_tgammaf(Float32 x);
/// Returns the integer part of self. This means that non-integer numbers are always truncated towards zero.
Float64 __llm_trunc(Float64 x);
/// Returns the integer part of self. This means that non-integer numbers are always truncated towards zero. (Float32)
Float32 __llm_truncf(Float32 x);
/// Returns the value of the least significant bit of the given floating point number.
Float64 __llm_ulp(Float64 x);
/// Bessel function of the second kind of order zero
Float64 __llm_y0(Float64 x);
/// Bessel function of the second kind of order zero (Float32)
Float32 __llm_y0f(Float32 x);
/// Bessel function of the second kind of order one
Float64 __llm_y1(Float64 x);
/// Bessel function of the second kind of order one (Float32)
Float32 __llm_y1f(Float32 x);
/// Bessel function of the second kind of order n
Float64 __llm_yn(Int n, Float64 x);
/// Bessel function of the second kind of order n (Float32)
Float32 __llm_ynf(Int n, Float32 x);

#ifdef __cplusplus
}
#endif // __cplusplus

#endif // _MATH_H
