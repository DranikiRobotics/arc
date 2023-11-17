use llm::{Float32, Float64, Radian32, Radian64};
type Int = i32;

macro_rules! bind {
    (
        #[$meta:meta]
        $func:ident ( $($name: ident: $type: ty),+ ) -> $out:ty;
        $($t:tt)*
    ) => (
        #[$meta]
        #[macros::ffi(@__llm_+$func)]
        pub extern "C" fn $func( $( $name: $type ),+ ) -> $out {
            <$out>::from(llm::$func( $( $name ),+ ))
        }
        bind!($($t)*);
    );
    () => ();
}

macro_rules! tup {
    ($name:ident $($k:ident:$v:ty),+) => (
        #[repr(C)]
        #[allow(non_camel_case_types)]
        pub struct $name {
            $( pub $k: $v ),+
        }
        impl $name {
            #[inline(always)]
            #[must_use = "this is a constructor"]
            pub const fn from(($($k),+): ($($v),+)) -> Self {
                Self { $($k),+ }
            }
        }
    );
}

tup!(Tuple_Float64_Int f: Float64, i: Int);
tup!(Tuple_Float32_Int f: Float32, i: Int);
tup!(Tuple_Float64_Float64 f1: Float64, f2: Float64);
tup!(Tuple_Float32_Float32 f1: Float32, f2: Float32);

bind! {
/// Arccosine
acos(x: Float64) -> Radian64;
/// Arccosine (Float32)
acosf(x: Float32) -> Radian32;
/// Inverse hyperbolic cosine
acosh(x: Float64) -> Float64;
/// Inverse hyperbolic cosine (Float32)
acoshf(x: Float32) -> Float32;
/// Arcsine
asin(x: Float64) -> Radian64;
/// Arcsine (Float32)
asinf(x: Float32) -> Radian32;
/// Inverse hyperbolic sine
asinh(x: Float64) -> Float64;
/// Inverse hyperbolic sine (Float32)
asinhf(x: Float32) -> Float32;
/// Arctangent
atan(x: Float64) -> Radian64;
/// Arctangent of y/x
atan2(y: Float64, x: Float64) -> Radian64;
/// Arctangent of y/x (Float32)
atan2f(y: Float32, x: Float32) -> Radian32;
/// Arctangent (Float32)
atanf(x: Float32) -> Radian32;
/// Inverse hyperbolic tangent
atanh(x: Float64) -> Float64;
/// Inverse hyperbolic tangent (Float32)
atanhf(x: Float32) -> Float32;
/// Computes the cube root of the argument.
cbrt(x: Float64) -> Float64;
/// Cube root (Float32)
cbrtf(x: Float32) -> Float32;
/// Ceil
ceil(x: Float64) -> Float64;
/// Ceil (Float32)
ceilf(x: Float32) -> Float32;
/// Sign of Y, magnitude of X
copysign(x: Float64, y: Float64) -> Float64;
/// Sign of Y, magnitude of X (Float32)
copysignf(x: Float32, y: Float32) -> Float32;
/// Cosine
cos(x: Radian64) -> Float64;
/// Cosine (Float32)
cosf(x: Radian32) -> Float32;
/// Hyperbolic cosine
cosh(x: Float64) -> Float64;
/// Hyperbolic cosine (Float32)
coshf(x: Float32) -> Float32;
/// Error function
erf(x: Float64) -> Float64;
/// Complementary error function
erfc(x: Float64) -> Float64;
/// Complementary error function (Float32)
erfcf(x: Float32) -> Float32;
/// Error function (Float32)
erff(x: Float32) -> Float32;
/// Exponential, base e
exp(x: Float64) -> Float64;
/// Exponential, base 2
exp2(x: Float64) -> Float64;
/// Exponential, base 2 (Float32)
exp2f(x: Float32) -> Float32;
/// Exponential, base 10
exp10(x: Float64) -> Float64;
/// Exponential, base 10 (Float32)
exp10f(x: Float32) -> Float32;
/// Exponential, base e (Float32)
expf(x: Float32) -> Float32;
/// Exponential, base e, of x-1
expm1(x: Float64) -> Float64;
/// Exponential, base e, of x-1 (Float32)
expm1f(x: Float32) -> Float32;
/// Absolute value (magnitude)
fabs(x: Float64) -> Float64;
/// Absolute value (magnitude) (Float32)
fabsf(x: Float32) -> Float32;
/// Positive difference
fdim(x: Float64, y: Float64) -> Float64;
/// Positive difference (Float32)
fdimf(x: Float32, y: Float32) -> Float32;
/// Floor
floor(x: Float64) -> Float64;
/// Floor (Float32)
floorf(x: Float32) -> Float32;
/// Floating multiply add
fma(x: Float64, y: Float64, z: Float64) -> Float64;
/// Fused multiply-add: Compute x * y + z with a single rounding error. (Float32)
fmaf(x: Float32, y: Float32, z: Float32) -> Float32;
/// Returns the maximum of the two arguments, signaling NaNs if either argument is a signaling NaN.
fmax(x: Float64, y: Float64) -> Float64;
/// Returns the maximum of the two arguments, signaling NaNs if either argument is a signaling NaN. (Float32)
fmaxf(x: Float32, y: Float32) -> Float32;
/// Returns the minimum of two numbers.
fmin(x: Float64, y: Float64) -> Float64;
/// Returns the minimum of two numbers. (Float32)
fminf(x: Float32, y: Float32) -> Float32;
/// Remainder of floating point division
fmod(x: Float64, y: Float64) -> Float64;
/// Remainder of floating point division (Float32)
fmodf(x: Float32, y: Float32) -> Float32;
/// Breaks the number into a normalized fraction and a base-2 exponent
frexp(x: Float64) -> Tuple_Float64_Int;
/// Breaks the number into a normalized fraction and a base-2 exponent (Float32)
frexpf(x: Float32) -> Tuple_Float32_Int;
/// Calculate the length of the hypotenuse of a right-angle triangle given legs of length x and y.
hypot(x: Float64, y: Float64) -> Float64;
/// Calculate the length of the hypotenuse of a right-angle triangle given legs of length x and y. (Float32)
hypotf(x: Float32, y: Float32) -> Float32;
/// Get exponent of floating point value
ilogb(x: Float64) -> Int;
/// Get exponent of floating point value (Float32)
ilogbf(x: Float32) -> Int;
/// Bessel function of the first kind of order zero
j0(x: Float64) -> Float64;
/// Bessel function of the first kind of order zero (Float32)
j0f(x: Float32) -> Float32;
/// Bessel function of the first kind of order one
j1(x: Float64) -> Float64;
/// Bessel function of the first kind of order one (Float32)
j1f(x: Float32) -> Float32;
/// Bessel function of the first kind of order zero of x.
jn(n: Int, x: Float64) -> Float64;
/// Bessel function of the first kind of order zero (Float32)
jnf(n: Int, x: Float32) -> Float32;
/// Returns x * 2^n.
ldexp(x: Float64, n: Int) -> Float64;
/// Returns x * 2^n. (Float32)
ldexpf(x: Float32, n: Int) -> Float32;
/// Natural logarithm of gamma function
lgamma(x: Float64) -> Float64;
/// Natural logarithm of gamma function (Float32)
lgammaf(x: Float32) -> Float32;
/// Natural logarithm of gamma function
lgamma_r(x: Float64) -> Tuple_Float64_Int;
/// Natural logarithm of gamma function (Float32)
lgammaf_r(x: Float32) -> Tuple_Float32_Int;
/// Return the natural logarithm of x.
ln(x: Float64) -> Float64;
/// Return the natural logarithm of x. (Float32)
lnf(x: Float32) -> Float32;
/// Returns the logarithm of x.
log(x: Float64) -> Float64;
/// Return the natural logarithm of 1+x.
log1p(x: Float64) -> Float64;
/// Return the natural logarithm of 1+x. (Float32)
log1pf(x: Float32) -> Float32;
/// Returns the base 2 logarithm of x.
log2(x: Float64) -> Float64;
/// Returns the base 2 logarithm of x. (Float32)
log2f(x: Float32) -> Float32;
/// Returns the base 10 logarithm of x.
log10(x: Float64) -> Float64;
/// Returns the base 10 logarithm of x. (Float32)
log10f(x: Float32) -> Float32;
/// Returns the logarithm of x
logf(x: Float32) -> Float32;
/// Breaks the given number into an integral and a fractional part.
modf(x: Float64) -> Tuple_Float64_Float64;
/// Breaks the given number into an integral and a fractional part. (Float32)
modff(x: Float32) -> Tuple_Float32_Float32;
/// Returns the next representable floating-point value following x in the direction of y.
nextafter(x: Float64, y: Float64) -> Float64;
/// Returns the next representable floating-point value following x in the direction of y. (Float32)
nextafterf(x: Float32, y: Float32) -> Float32;
/// Returns x raised to the power y.
pow(x: Float64, y: Float64) -> Float64;
/// Returns x raised to the power y. (Float32)
powf(x: Float32, y: Float32) -> Float32;
/// Returns the remainder of x/y.
remainder(x: Float64, y: Float64) -> Float64;
/// Returns the remainder of x/y. (Float32)
remainderf(x: Float32, y: Float32) -> Float32;
/// Return the remainder and part of the quotient of x and y.
remquo(x: Float64, y: Float64) -> Tuple_Float64_Int;
/// Return the remainder and part of the quotient of x and y. (Float32)
remquof(x: Float32, y: Float32) -> Tuple_Float32_Int;
/// Round to nearest integer, rounding halfway cases away from zero.
rint(x: Float64) -> Float64;
/// Round to nearest integer, rounding halfway cases away from zero. (Float32)
rintf(x: Float32) -> Float32;
/// Rounds x to the nearest integer in the direction of the current rounding mode.
round(x: Float64) -> Float64;
/// Rounds x to the nearest integer in the direction of the current rounding mode. (Float32)
roundf(x: Float32) -> Float32;
/// Returns x * 2^n
scalbn(x: Float64, n: Int) -> Float64;
/// Returns x * 2^n (Float32)
scalbnf(x: Float32, n: Int) -> Float32;
/// Returns the sine function of x.
sin(x: Radian64) -> Float64;
/// Simultaneously computes the sine and cosine of the argument x.
sincos(x: Radian64) -> Tuple_Float64_Float64;
/// Simultaneously computes the sine and cosine of the argument x. (Float32)
sincosf(x: Radian32) -> Tuple_Float32_Float32;
/// Returns the sine of the argument x.
sinf(x: Radian32) -> Float32;
/// Returns the hyperbolic sine of x.
sinh(x: Float64) -> Float64;
/// Returns the hyperbolic sine of x. (Float32)
sinhf(x: Float32) -> Float32;
/// Returns the square root of x.
sqrt(x: Float64) -> Float64;
/// Returns the square root of x. (Float32)
sqrtf(x: Float32) -> Float32;
/// Returns tangent of x.
tan(x: Radian64) -> Float64;
/// Returns tangent of x. (Float32)
tanf(x: Radian32) -> Float32;
/// Returns the hyperbolic tangent of x.
tanh(x: Float64) -> Float64;
/// Returns the hyperbolic tangent of x. (Float32)
tanhf(x: Float32) -> Float32;
/// Returns the gamma function of x.
tgamma(x: Float64) -> Float64;
/// Returns the gamma function of x. (Float32)
tgammaf(x: Float32) -> Float32;
/// Returns the integer part of self. This means that non-integer numbers are always truncated towards zero.
trunc(x: Float64) -> Float64;
/// Returns the integer part of self. This means that non-integer numbers are always truncated towards zero. (Float32)
truncf(x: Float32) -> Float32;
/// Returns the value of the least significant bit of the given floating point number.
ulp(x: Float64) -> Float64;
/// Bessel function of the second kind of order zero
y0(x: Float64) -> Float64;
/// Bessel function of the second kind of order zero (Float32)
y0f(x: Float32) -> Float32;
/// Bessel function of the second kind of order one
y1(x: Float64) -> Float64;
/// Bessel function of the second kind of order one (Float32)
y1f(x: Float32) -> Float32;
/// Bessel function of the second kind of order zero of x.
yn(n: Int, x: Float64) -> Float64;
/// Bessel function of the second kind of order zero (Float32)
ynf(n: Int, x: Float32) -> Float32;

}
