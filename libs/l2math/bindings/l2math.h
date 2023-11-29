/**
 * @file l2math.h
 * @author Матвей Т <https://matveit.dev>
 * @brief C bindings for l2math
 * @version 0.1.0-dev
 * @date 2023-11-22
*/

#ifndef __L2MATH_H
#define __L2MATH_H

#ifdef __cplusplus
namespace l2math {
    extern "C" {
#endif // __cplusplus

// C binding for rust's f64
typedef double ____l2math_f64;
// C binding for rust's f32
typedef float ____l2math_f32;
// C binding for rust's i32
typedef int ____l2math_i32;

/// High precision floating point number
typedef ____l2math_f64 __l2math_Float64;
/// Low precision floating point number
typedef ____l2math_f32 __l2math_Float32;
/// High precision floating point number representing an angle
typedef ____l2math_f64 __l2math_Radian64;
/// Low precision floating point number representing an angle
typedef ____l2math_f32 __l2math_Radian32;
/// High precision floating point number representing an angle in degrees
typedef ____l2math_f64 __l2math_Degree64;
/// Low precision floating point number representing an angle in degrees
typedef ____l2math_f32 __l2math_Degree32;
/// A standard signed integer
typedef ____l2math_i32 __l2math_Int;

/// A structure representing a tuple of a `__l2math_Float64` and an `__l2math_Int`
typedef struct __l2math_Tuple_Float64_Int {
    __l2math_Float64 f;
    __l2math_Int i;
} __l2math_Tuple_Float64_Int;

/// A structure representing a tuple of a `__l2math_Float32` and an `__l2math_Int`
typedef struct __l2math_Tuple_Float32_Int {
    __l2math_Float32 f;
    __l2math_Int i;
} __l2math_Tuple_Float32_Int;

/// A structure representing a tuple of two `__l2math_Float64`s
typedef struct __l2math_Tuple_Float64_Float64 {
    __l2math_Float64 f1;
    __l2math_Float64 f2;
} __l2math_Tuple_Float64_Float64;

/// A structure representing a tuple of two `__l2math_Float32`s
typedef struct __l2math_Tuple_Float32_Float32 {
    __l2math_Float32 f1;
    __l2math_Float32 f2;
} __l2math_Tuple_Float32_Float32;

/// Arccosine
__l2math_Radian64 __l2math_acos(__l2math_Float64 x);
/// Arccosine (__l2math_Float32)
__l2math_Radian32 __l2math_acosf(__l2math_Float32 x);
/// Inverse hyperbolic cosine
__l2math_Float64 __l2math_acosh(__l2math_Float64 x);
/// Inverse hyperbolic cosine (__l2math_Float32)
__l2math_Float32 __l2math_acoshf(__l2math_Float32 x);
/// Arcsine
__l2math_Radian64 __l2math_asin(__l2math_Float64 x);
/// Arcsine (__l2math_Float32)
__l2math_Radian32 __l2math_asinf(__l2math_Float32 x);
/// Inverse hyperbolic sine
__l2math_Float64 __l2math_asinh(__l2math_Float64 x);
/// Inverse hyperbolic sine (__l2math_Float32)
__l2math_Float32 __l2math_asinhf(__l2math_Float32 x);
/// Arctangent
__l2math_Radian64 __l2math_atan(__l2math_Float64 x);
/// Arctangent of y/x
__l2math_Radian64 __l2math_atan2(__l2math_Float64 y, __l2math_Float64 x);
/// Arctangent of y/x (__l2math_Float32)
__l2math_Radian32 __l2math_atan2f(__l2math_Float32 y, __l2math_Float32 x);
/// Arctangent (__l2math_Float32)
__l2math_Radian32 __l2math_atanf(__l2math_Float32 x);
/// Inverse hyperbolic tangent
__l2math_Float64 __l2math_atanh(__l2math_Float64 x);
/// Inverse hyperbolic tangent (__l2math_Float32)
__l2math_Float32 __l2math_atanhf(__l2math_Float32 x);
/// Computes the cube root of the argument.
__l2math_Float64 __l2math_cbrt(__l2math_Float64 x);
/// Cube root (__l2math_Float32)
__l2math_Float32 __l2math_cbrtf(__l2math_Float32 x);
/// Ceil
__l2math_Float64 __l2math_ceil(__l2math_Float64 x);
/// Ceil (__l2math_Float32)
__l2math_Float32 __l2math_ceilf(__l2math_Float32 x);
/// Sign of Y, magnitude of X
__l2math_Float64 __l2math_copysign(__l2math_Float64 x, __l2math_Float64 y);
/// Sign of Y, magnitude of X (__l2math_Float32)
__l2math_Float32 __l2math_copysignf(__l2math_Float32 x, __l2math_Float32 y);
/// Cosine
__l2math_Float64 __l2math_cos(__l2math_Radian64 x);
/// Cosine (__l2math_Float32)
__l2math_Float32 __l2math_cosf(__l2math_Radian32 x);
/// Hyperbolic cosine
__l2math_Float64 __l2math_cosh(__l2math_Float64 x);
/// Hyperbolic cosine (__l2math_Float32)
__l2math_Float32 __l2math_coshf(__l2math_Float32 x);
/// Computes the exponential function of x.
__l2math_Float64 __l2math_exp(__l2math_Float64 x);
/// Computes the exponential function of x (__l2math_Float32).
__l2math_Float32 __l2math_expf(__l2math_Float32 x);
/// Computes the exponential minus one function of x.
__l2math_Float64 __l2math_expm1(__l2math_Float64 x);
/// Computes the exponential minus one function of x (__l2math_Float32).
__l2math_Float32 __l2math_expm1f(__l2math_Float32 x);
/// Absolute value
__l2math_Float64 __l2math_fabs(__l2math_Float64 x);
/// Absolute value (__l2math_Float32)
__l2math_Float32 __l2math_fabsf(__l2math_Float32 x);
/// Factorial
__l2math_Float64 __l2math_factorial(__l2math_Float64 x);
/// Factorial (__l2math_Float32)
__l2math_Float32 __l2math_factorialf(__l2math_Float32 x);
/// Floor
__l2math_Float64 __l2math_floor(__l2math_Float64 x);
/// Floor (__l2math_Float32)
__l2math_Float32 __l2math_floorf(__l2math_Float32 x);
/// Computes the fused multiply-add of x, y and z.
__l2math_Float64 __l2math_fma(__l2math_Float64 x, __l2math_Float64 y, __l2math_Float64 z);
/// Computes the fused multiply-add of x, y and z (__l2math_Float32).
__l2math_Float32 __l2math_fmaf(__l2math_Float32 x, __l2math_Float32 y, __l2math_Float32 z);
/// Returns the maximum of the two arguments, signaling NaNs if either argument is a signaling NaN.
__l2math_Float64 __l2math_fmax(__l2math_Float64 x, __l2math_Float64 y);
/// Returns the maximum of the two arguments, signaling NaNs if either argument is a signaling NaN (__l2math_Float32).
__l2math_Float32 __l2math_fmaxf(__l2math_Float32 x, __l2math_Float32 y);
/// Returns the minimum of the two arguments, signaling NaNs if either argument is a signaling NaN.
__l2math_Float64 __l2math_fmin(__l2math_Float64 x, __l2math_Float64 y);
/// Returns the minimum of the two arguments, signaling NaNs if either argument is a signaling NaN (__l2math_Float32).
__l2math_Float32 __l2math_fminf(__l2math_Float32 x, __l2math_Float32 y);
/// Computes the floating-point remainder of dividing x by y.
__l2math_Float64 __l2math_fmod(__l2math_Float64 x, __l2math_Float64 y);
/// Computes the floating-point remainder of dividing x by y (__l2math_Float32).
__l2math_Float32 __l2math_fmodf(__l2math_Float32 x, __l2math_Float32 y);
/// Breaks the number into a normalized fraction and a base-2 exponent
__l2math_Tuple_Float64_Int __l2math_frexp(__l2math_Float64 x);
/// Breaks the number into a normalized fraction and a base-2 exponent (__l2math_Float32)
__l2math_Tuple_Float32_Int __l2math_frexpf(__l2math_Float32 x);
/// Calculate the length of the hypotenuse of a right-angle triangle given legs of length x and y.
__l2math_Float64 __l2math_hypot(__l2math_Float64 x, __l2math_Float64 y);
/// Calculate the length of the hypotenuse of a right-angle triangle given legs of length x and y (__l2math_Float32).
__l2math_Float32 __l2math_hypotf(__l2math_Float32 x, __l2math_Float32 y);
/// Get exponent of floating point value
__l2math_Int __l2math_ilogb(__l2math_Float64 x);
/// Get exponent of floating point value (__l2math_Float32)
__l2math_Int __l2math_ilogbf(__l2math_Float32 x);
/// Bessel function of the first kind of order zero
__l2math_Float64 __l2math_j0(__l2math_Float64 x);
/// Bessel function of the first kind of order zero (__l2math_Float32)
__l2math_Float32 __l2math_j0f(__l2math_Float32 x);
/// Bessel function of the first kind of order one
__l2math_Float64 __l2math_j1(__l2math_Float64 x);
/// Bessel function of the first kind of order one (__l2math_Float32)
__l2math_Float32 __l2math_j1f(__l2math_Float32 x);
/// Bessel function of the first kind of order n
__l2math_Float64 __l2math_jn(__l2math_Int n, __l2math_Float64 x);
/// Bessel function of the first kind of order n (__l2math_Float32)
__l2math_Float32 __l2math_jnf(__l2math_Int n, __l2math_Float32 x);
/// Returns x * 2^n.
__l2math_Float64 __l2math_ldexp(__l2math_Float64 x, __l2math_Int n);
/// Returns x * 2^n (__l2math_Float32).
__l2math_Float32 __l2math_ldexpf(__l2math_Float32 x, __l2math_Int n);
/// Natural logarithm of gamma function
__l2math_Float64 __l2math_lgamma(__l2math_Float64 x);
/// Natural logarithm of gamma function (__l2math_Float32)
__l2math_Float32 __l2math_lgammaf(__l2math_Float32 x);
/// Natural logarithm of gamma function
__l2math_Tuple_Float64_Int __l2math_lgamma_r(__l2math_Float64 x);
/// Natural logarithm of gamma function (__l2math_Float32)
__l2math_Tuple_Float32_Int __l2math_lgammaf_r(__l2math_Float32 x);
/// Return the natural logarithm of x.
__l2math_Float64 __l2math_ln(__l2math_Float64 x);
/// Return the natural logarithm of x (__l2math_Float32).
__l2math_Float32 __l2math_lnf(__l2math_Float32 x);
/// Return logarithm of x.
__l2math_Float64 __l2math_log(__l2math_Float64 x);
/// Return logarithm of x (__l2math_Float32).
__l2math_Float32 __l2math_logf(__l2math_Float32 x);
/// Return logarithm of x to base 10.
__l2math_Float64 __l2math_log10(__l2math_Float64 x);
/// Return logarithm of x to base 10 (__l2math_Float32).
__l2math_Float32 __l2math_log10f(__l2math_Float32 x);
/// Return logarithm of x to base 2.
__l2math_Float64 __l2math_log2(__l2math_Float64 x);
/// Return logarithm of x to base 2 (__l2math_Float32).
__l2math_Float32 __l2math_log2f(__l2math_Float32 x);
/// Return the natural logarithm of one plus x.
__l2math_Float64 __l2math_log1p(__l2math_Float64 x);
/// Return the natural logarithm of one plus x (__l2math_Float32).
__l2math_Float32 __l2math_log1pf(__l2math_Float32 x);
/// Breaks the given number into an integral and a fractional part.
__l2math_Tuple_Float64_Float64 __l2math_modf(__l2math_Float64 x);
/// Breaks the given number into an integral and a fractional part (__l2math_Float32).
__l2math_Tuple_Float32_Float32 __l2math_modff(__l2math_Float32 x);
/// Returns the next representable floating-point value following x in the direction of y.
__l2math_Float64 __l2math_nextafter(__l2math_Float64 x, __l2math_Float64 y);
/// Returns the next representable floating-point value following x in the direction of y. (__l2math_Float32)
__l2math_Float32 __l2math_nextafterf(__l2math_Float32 x, __l2math_Float32 y);
/// Returns x raised to the power y.
__l2math_Float64 __l2math_pow(__l2math_Float64 x, __l2math_Float64 y);
/// Returns x raised to the power y. (__l2math_Float32)
__l2math_Float32 __l2math_powf(__l2math_Float32 x, __l2math_Float32 y);
/// Returns the remainder of x/y.
__l2math_Float64 __l2math_remainder(__l2math_Float64 x, __l2math_Float64 y);
/// Returns the remainder of x/y. (__l2math_Float32)
__l2math_Float32 __l2math_remainderf(__l2math_Float32 x, __l2math_Float32 y);
/// Return the remainder and part of the quotient of x and y.
__l2math_Tuple_Float64_Int __l2math_remquo(__l2math_Float64 x, __l2math_Float64 y);
/// Return the remainder and part of the quotient of x and y. (__l2math_Float32)
__l2math_Tuple_Float32_Int __l2math_remquof(__l2math_Float32 x, __l2math_Float32 y);
/// Round to nearest integer, rounding halfway cases away from zero.
__l2math_Float64 __l2math_rint(__l2math_Float64 x);
/// Round to nearest integer, rounding halfway cases away from zero. (__l2math_Float32)
__l2math_Float32 __l2math_rintf(__l2math_Float32 x);
/// Rounds x to the nearest integer in the direction of the current rounding mode.
__l2math_Float64 __l2math_round(__l2math_Float64 x);
/// Rounds x to the nearest integer in the direction of the current rounding mode. (__l2math_Float32)
__l2math_Float32 __l2math_roundf(__l2math_Float32 x);
/// Returns x * 2^n
__l2math_Float64 __l2math_scalbn(__l2math_Float64 x, __l2math_Int n);
/// Returns x * 2^n (__l2math_Float32)
__l2math_Float32 __l2math_scalbnf(__l2math_Float32 x, __l2math_Int n);
/// Returns the sine function of x.
__l2math_Float64 __l2math_sin(__l2math_Radian64 x);
/// Simultaneously computes the sine and cosine of the argument x.
__l2math_Tuple_Float64_Float64 __l2math_sincos(__l2math_Radian64 x);
/// Simultaneously computes the sine and cosine of the argument x. (__l2math_Float32)
__l2math_Tuple_Float32_Float32 __l2math_sincosf(__l2math_Radian32 x);
/// Returns the sine of the argument x.
__l2math_Float64 __l2math_sinf(__l2math_Float64 x);
/// Returns the hyperbolic sine of x.
__l2math_Float64 __l2math_sinh(__l2math_Float64 x);
/// Returns the hyperbolic sine of x. (__l2math_Float32)
__l2math_Float32 __l2math_sinhf(__l2math_Float32 x);
/// Returns the square root of x.
__l2math_Float64 __l2math_sqrt(__l2math_Float64 x);
/// Returns the square root of x. (__l2math_Float32)
__l2math_Float32 __l2math_sqrtf(__l2math_Float32 x);
/// Returns tangent of x.
__l2math_Float64 __l2math_tan(__l2math_Radian64 x);
/// Returns tangent of x. (__l2math_Float32)
__l2math_Float32 __l2math_tanf(__l2math_Radian32 x);
/// Returns the hyperbolic tangent of x.
__l2math_Float64 __l2math_tanh(__l2math_Float64 x);
/// Returns the hyperbolic tangent of x. (__l2math_Float32)
__l2math_Float32 __l2math_tanhf(__l2math_Float32 x);
/// Returns the gamma function of x.
__l2math_Float64 __l2math_tgamma(__l2math_Float64 x);
/// Returns the gamma function of x. (__l2math_Float32)
__l2math_Float32 __l2math_tgammaf(__l2math_Float32 x);
/// Returns the integer part of self. This means that non-integer numbers are always truncated towards zero.
__l2math_Float64 __l2math_trunc(__l2math_Float64 x);
/// Returns the integer part of self. This means that non-integer numbers are always truncated towards zero. (__l2math_Float32)
__l2math_Float32 __l2math_truncf(__l2math_Float32 x);
/// Returns the value of the least significant bit of the given floating point number.
__l2math_Float64 __l2math_ulp(__l2math_Float64 x);
/// Bessel function of the second kind of order zero
__l2math_Float64 __l2math_y0(__l2math_Float64 x);
/// Bessel function of the second kind of order zero (__l2math_Float32)
__l2math_Float32 __l2math_y0f(__l2math_Float32 x);
/// Bessel function of the second kind of order one
__l2math_Float64 __l2math_y1(__l2math_Float64 x);
/// Bessel function of the second kind of order one (__l2math_Float32)
__l2math_Float32 __l2math_y1f(__l2math_Float32 x);
/// Bessel function of the second kind of order n
__l2math_Float64 __l2math_yn(__l2math_Int n, __l2math_Float64 x);
/// Bessel function of the second kind of order n (__l2math_Float32)
__l2math_Float32 __l2math_ynf(__l2math_Int n, __l2math_Float32 x);

#ifdef __cplusplus
    }

    /// High precision floating point number
    typedef __l2math_Float64 Float64;
    /// Low precision floating point number
    typedef __l2math_Float32 Float32;
    /// High precision floating point number representing an angle
    typedef __l2math_Radian64 Radian64;
    /// Low precision floating point number representing an angle
    typedef __l2math_Radian32 Radian32;
    /// High precision floating point number representing an angle in degrees
    typedef __l2math_Degree64 Degree64;
    /// Low precision floating point number representing an angle in degrees
    typedef __l2math_Degree32 Degree32;
    /// A standard signed integer
    typedef __l2math_Int Int;

    constexpr const Float64 PI = 3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679;
    constexpr const Float64 _PI_OVER_180 = 0.0174532925199432957692369076848861271344287188854172545609719144017100911460344944368224156963450948221230449250737905924838546922752810123984742189340471173191682450150107695616975535812386053051687886912711720870329635896026424901877043509181733439396980475940192241589469684813789632978181124952292984699278144795310454160084495609046069671761964687105143908889518362808267803695632452608441195089412947626131431088441838454784298996256210728062141559692354442374975963993652929160623774343500663840546315186802258;
    constexpr const Float64 _180_OVER_PI = 57.295779513082320876798154814105170332405472466564321549160243861202847148321552632440968995851110944186223381632864893281448264601248315036068267863411942122526388097467267926307988702893110767938261442638263158209610460487020506444259656841120171912057738566280431284962624203376187937297623870790340315980719624089522045186205459923396314841906966220115126609691801514787637366923164107126774038514690165499594192515711986479435210661624389035202306756177796757113315683506205731313360156501348898018788709917776439;

    constexpr const Radian64 DEG2RAD(Degree64 deg) {
        return deg * _PI_OVER_180;
    }

    constexpr const Degree64 RAD2DEG(Radian64 rad) {
        return rad * _180_OVER_PI;
    }

    template<typename L, typename R>
    class Tuple {
    private:
        L _l;
        R _r;
    public:
        constexpr Tuple(L l, R r) : _l(l), _r(r) {}
        constexpr const inline L l() const {
            return this->_l;
        }
        constexpr const inline R r() const {
            return this->_r;
        }
    };
    /// Arccosine
    constexpr const inline Radian64 acos(Float64 x) {
        return __l2math_acos(x);
    }
    /// Arccosine (Float32)
    constexpr const inline Radian32 acos(Float32 x) {
        return __l2math_acosf(x);
    }
    /// Inverse hyperbolic cosine
    constexpr const inline Float64 acosh(Float64 x) {
        return __l2math_acosh(x);
    }
    /// Inverse hyperbolic cosine (Float32)
    constexpr const inline Float32 acosh(Float32 x) {
        return __l2math_acoshf(x);
    }
    /// Arcsine
    constexpr const inline Radian64 asin(Float64 x) {
        return __l2math_asin(x);
    }
    /// Arcsine (Float32)
    constexpr const inline Radian32 asin(Float32 x) {
        return __l2math_asinf(x);
    }
    /// Inverse hyperbolic sine
    constexpr const inline Float64 asinh(Float64 x) {
        return __l2math_asinh(x);
    }
    /// Inverse hyperbolic sine (Float32)
    constexpr const inline Float32 asinh(Float32 x) {
        return __l2math_asinhf(x);
    }
    /// Arctangent
    constexpr const inline Radian64 atan(Float64 x) {
        return __l2math_atan(x);
    }
    /// Arctangent of y/x
    constexpr const inline Radian64 atan2(Float64 y, Float64 x) {
        return __l2math_atan2(y, x);
    }
    /// Arctangent of y/x (Float32)
    constexpr const inline Radian32 atan2(Float32 y, Float32 x) {
        return __l2math_atan2f(y, x);
    }
    /// Arctangent (Float32)
    constexpr const inline Radian32 atan(Float32 x) {
        return __l2math_atanf(x);
    }
    /// Inverse hyperbolic tangent
    constexpr const inline Float64 atanh(Float64 x) {
        return __l2math_atanh(x);
    }
    /// Inverse hyperbolic tangent (Float32)
    constexpr const inline Float32 atanh(Float32 x) {
        return __l2math_atanhf(x);
    }
    /// Computes the cube root of the argument.
    constexpr const inline Float64 cbrt(Float64 x) {
        return __l2math_cbrt(x);
    }
    /// Cube root (Float32)
    constexpr const inline Float32 cbrt(Float32 x) {
        return __l2math_cbrtf(x);
    }
    /// Ceil
    constexpr const inline Float64 ceil(Float64 x) {
        return __l2math_ceil(x);
    }
    /// Ceil (Float32)
    constexpr const inline Float32 ceil(Float32 x) {
        return __l2math_ceilf(x);
    }
    /// Sign of Y, magnitude of X
    constexpr const inline Float64 copysign(Float64 x, Float64 y) {
        return __l2math_copysign(x, y);
    }
    /// Sign of Y, magnitude of X (Float32)
    constexpr const inline Float32 copysign(Float32 x, Float32 y) {
        return __l2math_copysignf(x, y);
    }
    /// Cosine
    constexpr const inline Float64 cos(Radian64 x) {
        return __l2math_cos(x);
    }
    /// Cosine (Float32)
    constexpr const inline Float32 cos(Radian32 x) {
        return __l2math_cosf(x);
    }
    /// Hyperbolic cosine
    constexpr const inline Float64 cosh(Float64 x) {
        return __l2math_cosh(x);
    }
    /// Hyperbolic cosine (Float32)
    constexpr const inline Float32 cosh(Float32 x) {
        return __l2math_coshf(x);
    }
    /// Computes the exponential function of x.
    constexpr const inline Float64 exp(Float64 x) {
        return __l2math_exp(x);
    }
    /// Computes the exponential function of x (Float32).
    constexpr const inline Float32 exp(Float32 x) {
        return __l2math_expf(x);
    }
    /// Computes the exponential minus one function of x.
    constexpr const inline Float64 expm1(Float64 x) {
        return __l2math_expm1(x);
    }
    /// Computes the exponential minus one function of x (Float32).
    constexpr const inline Float32 expm1(Float32 x) {
        return __l2math_expm1f(x);
    }
    /// Absolute value
    constexpr const inline Float64 abs(Float64 x) {
        return __l2math_fabs(x);
    }
    /// Absolute value (Float32)
    constexpr const inline Float32 abs(Float32 x) {
        return __l2math_fabsf(x);
    }
    /// Factorial
    constexpr const inline Float64 factorial(Float64 x) {
        return __l2math_factorial(x);
    }
    /// Factorial (Float32)
    constexpr const inline Float32 factorial(Float32 x) {
        return __l2math_factorialf(x);
    }
    /// Floor
    constexpr const inline Float64 floor(Float64 x) {
        return __l2math_floor(x);
    }
    /// Floor (Float32)
    constexpr const inline Float32 floor(Float32 x) {
        return __l2math_floorf(x);
    }
    /// Computes the fused multiply-add of x, y and z.
    constexpr const inline Float64 fma(Float64 x, Float64 y, Float64 z) {
        return __l2math_fma(x, y, z);
    }
    /// Computes the fused multiply-add of x, y and z (Float32).
    constexpr const inline Float32 fma(Float32 x, Float32 y, Float32 z) {
        return __l2math_fmaf(x, y, z);
    }
    /// Returns the maximum of the two arguments, signaling NaNs if either argument is a signaling NaN.
    constexpr const inline Float64 max(Float64 x, Float64 y) {
        return __l2math_fmax(x, y);
    }
    /// Returns the maximum of the two arguments, signaling NaNs if either argument is a signaling NaN (Float32).
    constexpr const inline Float32 max(Float32 x, Float32 y) {
        return __l2math_fmaxf(x, y);
    }
    /// Returns the minimum of the two arguments, signaling NaNs if either argument is a signaling NaN.
    constexpr const inline Float64 min(Float64 x, Float64 y) {
        return __l2math_fmin(x, y);
    }
    /// Returns the minimum of the two arguments, signaling NaNs if either argument is a signaling NaN (Float32).
    constexpr const inline Float32 min(Float32 x, Float32 y) {
        return __l2math_fminf(x, y);
    }
    /// Computes the floating-point remainder of dividing x by y.
    constexpr const inline Float64 mod(Float64 x, Float64 y) {
        return __l2math_fmod(x, y);
    }
    /// Computes the floating-point remainder of dividing x by y (Float32).
    constexpr const inline Float32 mod(Float32 x, Float32 y) {
        return __l2math_fmodf(x, y);
    }
    /// Breaks the number into a normalized fraction and a base-2 exponent
    constexpr const inline Tuple<Float64, Int> frexp(Float64 x) {
        __l2math_Tuple_Float64_Int res = __l2math_frexp(x);
        return Tuple<Float64, Int>(res.f, res.i);
    }
    /// Breaks the number into a normalized fraction and a base-2 exponent (Float32)
    constexpr const inline Tuple<Float32, Int> frexp(Float32 x) {
        __l2math_Tuple_Float32_Int res = __l2math_frexpf(x);
        return Tuple<Float32, Int>(res.f, res.i);
    }
    /// Calculate the length of the hypotenuse of a right-angle triangle given legs of length x and y.
    constexpr const inline Float64 hypot(Float64 x, Float64 y) {
        return __l2math_hypot(x, y);
    }
    /// Calculate the length of the hypotenuse of a right-angle triangle given legs of length x and y (Float32).
    constexpr const inline Float32 hypot(Float32 x, Float32 y) {
        return __l2math_hypotf(x, y);
    }
    /// Get exponent of floating point value
    constexpr const inline Int ilogb(Float64 x) {
        return __l2math_ilogb(x);
    }
    /// Get exponent of floating point value (Float32)
    constexpr const inline Int ilogb(Float32 x) {
        return __l2math_ilogbf(x);
    }
    /// Bessel function of the first kind of order zero
    constexpr const inline Float64 j0(Float64 x) {
        return __l2math_j0(x);
    }
    /// Bessel function of the first kind of order zero (Float32)
    constexpr const inline Float32 j0(Float32 x) {
        return __l2math_j0f(x);
    }
    /// Bessel function of the first kind of order one
    constexpr const inline Float64 j1(Float64 x) {
        return __l2math_j1(x);
    }
    /// Bessel function of the first kind of order one (Float32)
    constexpr const inline Float32 j1(Float32 x) {
        return __l2math_j1f(x);
    }
    /// Bessel function of the first kind of order n
    constexpr const inline Float64 jn(Int n, Float64 x) {
        return __l2math_jn(n, x);
    }
    /// Bessel function of the first kind of order n (Float32)
    constexpr const inline Float32 jn(Int n, Float32 x) {
        return __l2math_jnf(n, x);
    }
    /// Returns x * 2^n.
    constexpr const inline Float64 ldexp(Float64 x, Int n) {
        return __l2math_ldexp(x, n);
    }
    /// Returns x * 2^n (Float32).
    constexpr const inline Float32 ldexp(Float32 x, Int n) {
        return __l2math_ldexpf(x, n);
    }
    /// Natural logarithm of gamma function
    constexpr const inline Float64 lgamma(Float64 x) {
        return __l2math_lgamma(x);
    }
    /// Natural logarithm of gamma function (Float32)
    constexpr const inline Float32 lgamma(Float32 x) {
        return __l2math_lgammaf(x);
    }
    /// Natural logarithm of gamma function
    constexpr const inline Tuple<Float64, Int> lgamma_r(Float64 x) {
        __l2math_Tuple_Float64_Int res = __l2math_lgamma_r(x);
        return Tuple<Float64, Int>(res.f, res.i);
    }
    /// Natural logarithm of gamma function (Float32)
    constexpr const inline Tuple<Float32, Int> lgamma_r(Float32 x) {
        __l2math_Tuple_Float32_Int res = __l2math_lgammaf_r(x);
        return Tuple<Float32, Int>(res.f, res.i);
    }
    /// Return the natural logarithm of x.
    constexpr const inline Float64 ln(Float64 x) {
        return __l2math_ln(x);
    }
    /// Return the natural logarithm of x (Float32).
    constexpr const inline Float32 ln(Float32 x) {
        return __l2math_lnf(x);
    }
    /// Return logarithm of x.
    constexpr const inline Float64 log(Float64 x) {
        return __l2math_log(x);
    }
    /// Return logarithm of x (Float32).
    constexpr const inline Float32 log(Float32 x) {
        return __l2math_logf(x);
    }
    /// Return logarithm of x to base 10.
    constexpr const inline Float64 log10(Float64 x) {
        return __l2math_log10(x);
    }
    /// Return logarithm of x to base 10 (Float32).
    constexpr const inline Float32 log10(Float32 x) {
        return __l2math_log10f(x);
    }
    /// Return logarithm of x to base 2.
    constexpr const inline Float64 log2(Float64 x) {
        return __l2math_log2(x);
    }
    /// Return logarithm of x to base 2 (Float32).
    constexpr const inline Float32 log2(Float32 x) {
        return __l2math_log2f(x);
    }
    /// Return the natural logarithm of one plus x.
    constexpr const inline Float64 log1p(Float64 x) {
        return __l2math_log1p(x);
    }
    /// Return the natural logarithm of one plus x (Float32).
    constexpr const inline Float32 log1p(Float32 x) {
        return __l2math_log1pf(x);
    }
    /// Breaks the given number into an integral and a fractional part.
    constexpr const inline Tuple<Float64, Float64> mod(Float64 x) {
        __l2math_Tuple_Float64_Float64 res = __l2math_modf(x);
        return Tuple<Float64, Float64>(res.f1, res.f2);
    }
    /// Breaks the given number into an integral and a fractional part (Float32).
    constexpr const inline Tuple<Float32, Float32> mod(Float32 x) {
        __l2math_Tuple_Float32_Float32 res = __l2math_modff(x);
        return Tuple<Float32, Float32>(res.f1, res.f2);
    }
    /// Returns the next representable floating-point value following x in the direction of y.
    constexpr const inline Float64 nextafter(Float64 x, Float64 y) {
        return __l2math_nextafter(x, y);
    }
    /// Returns the next representable floating-point value following x in the direction of y. (Float32)
    constexpr const inline Float32 nextafter(Float32 x, Float32 y) {
        return __l2math_nextafterf(x, y);
    }
    /// Returns x raised to the power y.
    constexpr const inline Float64 pow(Float64 x, Float64 y) {
        return __l2math_pow(x, y);
    }
    /// Returns x raised to the power y. (Float32)
    constexpr const inline Float32 pow(Float32 x, Float32 y) {
        return __l2math_powf(x, y);
    }
    /// Returns the remainder of x/y.
    constexpr const inline Float64 remainder(Float64 x, Float64 y) {
        return __l2math_remainder(x, y);
    }
    /// Returns the remainder of x/y. (Float32)
    constexpr const inline Float32 remainder(Float32 x, Float32 y) {
        return __l2math_remainderf(x, y);
    }
    /// Return the remainder and part of the quotient of x and y.
    constexpr const inline Tuple<Float64, Int> remquo(Float64 x, Float64 y) {
        __l2math_Tuple_Float64_Int res = __l2math_remquo(x, y);
        return Tuple<Float64, Int>(res.f, res.i);
    }
    /// Return the remainder and part of the quotient of x and y. (Float32)
    constexpr const inline Tuple<Float32, Int> remquo(Float32 x, Float32 y) {
        __l2math_Tuple_Float32_Int res = __l2math_remquof(x, y);
        return Tuple<Float32, Int>(res.f, res.i);
    }
    /// Round to nearest integer, rounding halfway cases away from zero.
    constexpr const inline Float64 rint(Float64 x) {
        return __l2math_rint(x);
    }
    /// Round to nearest integer, rounding halfway cases away from zero. (Float32)
    constexpr const inline Float32 rint(Float32 x) {
        return __l2math_rintf(x);
    }
    /// Rounds x to the nearest integer in the direction of the current rounding mode.
    constexpr const inline Float64 round(Float64 x) {
        return __l2math_round(x);
    }
    /// Rounds x to the nearest integer in the direction of the current rounding mode. (Float32)
    constexpr const inline Float32 round(Float32 x) {
        return __l2math_roundf(x);
    }
    /// Rounds x to a number of digits after the decimal point.
    constexpr const inline Float64 round(Float64 x, Float64 to) {
        return round(x / to) * to;
    }
    /// Rounds x to a number of digits after the decimal point. (Float32)
    constexpr const inline Float32 round(Float32 x, Float32 to) {
        return round(x / to) * to;
    }
    /// Returns x * 2^n
    constexpr const inline Float64 scalbn(Float64 x, Int n) {
        return __l2math_scalbn(x, n);
    }
    /// Returns x * 2^n (Float32)
    constexpr const inline Float32 scalbn(Float32 x, Int n) {
        return __l2math_scalbnf(x, n);
    }
    /// Returns the sine function of x.
    constexpr const inline Float64 sin(Radian64 x) {
        return __l2math_sin(x);
    }
    /// Simultaneously computes the sine and cosine of the argument x.
    constexpr const inline Tuple<Float64, Float64> sincos(Radian64 x) {
        __l2math_Tuple_Float64_Float64 res = __l2math_sincos(x);
        return Tuple<Float64, Float64>(res.f1, res.f2);
    }
    /// Simultaneously computes the sine and cosine of the argument x. (Float32)
    constexpr const inline Tuple<Float32, Float32> sincos(Radian32 x) {
        __l2math_Tuple_Float32_Float32 res = __l2math_sincosf(x);
        return Tuple<Float32, Float32>(res.f1, res.f2);
    }
    /// Returns the sine of the argument x.
    constexpr const inline Float32 sin(Radian32 x) {
        return __l2math_sinf(x);
    }
    /// Returns the hyperbolic sine of x.
    constexpr const inline Float64 sinh(Float64 x) {
        return __l2math_sinh(x);
    }
    /// Returns the hyperbolic sine of x. (Float32)
    constexpr const inline Float32 sinh(Float32 x) {
        return __l2math_sinhf(x);
    }
    /// Returns the square root of x.
    constexpr const inline Float64 sqrt(Float64 x) {
        return __l2math_sqrt(x);
    }
    /// Returns the square root of x. (Float32)
    constexpr const inline Float32 sqrt(Float32 x) {
        return __l2math_sqrtf(x);
    }
    /// Returns tangent of x.
    constexpr const inline Float64 tan(Radian64 x) {
        return __l2math_tan(x);
    }
    /// Returns tangent of x. (Float32)
    constexpr const inline Float32 tan(Radian32 x) {
        return __l2math_tanf(x);
    }
    /// Returns the hyperbolic tangent of x.
    constexpr const inline Float64 tanh(Float64 x) {
        return __l2math_tanh(x);
    }
    /// Returns the hyperbolic tangent of x. (Float32)
    constexpr const inline Float32 tanh(Float32 x) {
        return __l2math_tanhf(x);
    }
    /// Returns the gamma function of x.
    constexpr const inline Float64 tgamma(Float64 x) {
        return __l2math_tgamma(x);
    }
    /// Returns the gamma function of x. (Float32)
    constexpr const inline Float32 tgamma(Float32 x) {
        return __l2math_tgammaf(x);
    }
    /// Returns the integer part of self. This means that non-integer numbers are always truncated towards zero.
    constexpr const inline Float64 trunc(Float64 x) {
        return __l2math_trunc(x);
    }
    /// Returns the integer part of self. This means that non-integer numbers are always truncated towards zero. (Float32)
    constexpr const inline Float32 trunc(Float32 x) {
        return __l2math_truncf(x);
    }
    /// Returns the value of the least significant bit of the given floating point number.
    constexpr const inline Float64 ulp(Float64 x) {
        return __l2math_ulp(x);
    }
    /// Bessel function of the second kind of order zero
    constexpr const inline Float64 y0(Float64 x) {
        return __l2math_y0(x);
    }
    /// Bessel function of the second kind of order zero (Float32)
    constexpr const inline Float32 y0(Float32 x) {
        return __l2math_y0f(x);
    }
    /// Bessel function of the second kind of order one
    constexpr const inline Float64 y1(Float64 x) {
        return __l2math_y1(x);
    }
    /// Bessel function of the second kind of order one (Float32)
    constexpr const inline Float32 y1(Float32 x) {
        return __l2math_y1f(x);
    }
    /// Bessel function of the second kind of order n
    constexpr const inline Float64 yn(Int n, Float64 x) {
        return __l2math_yn(n, x);
    }
    /// Bessel function of the second kind of order n (Float32)
    constexpr const inline Float32 yn(Int n, Float32 x) {
        return __l2math_ynf(n, x);
    }
}
#endif // __cplusplus

#endif // __L2MATH_BINDINGS_H
