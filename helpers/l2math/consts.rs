#![allow(clippy::excessive_precision)]

use crate::{Float64, Float32};

/// Archimedes' constant (π)
pub const PI: Float64 = 3.14159265358979323846264338327950288_f64;

/// The full circle constant (τ)
///
/// Equal to 2π.
pub const TAU: Float64 = 6.28318530717958647692528676655900577_f64;

/// The golden ratio (φ)
pub const PHI: Float64 = 1.618033988749894848204586834365638118_f64;

/// The Euler-Mascheroni constant (γ)
pub const EGAMMA: Float64 = 0.577215664901532860606512090082402431_f64;

/// π/2
pub const FRAC_PI_2: Float64 = 1.57079632679489661923132169163975144_f64;

/// π/3
pub const FRAC_PI_3: Float64 = 1.04719755119659774615421446109316763_f64;

/// π/4
pub const FRAC_PI_4: Float64 = 0.785398163397448309615660845819875721_f64;

/// π/6
pub const FRAC_PI_6: Float64 = 0.52359877559829887307710723054658381_f64;

/// π/8
pub const FRAC_PI_8: Float64 = 0.39269908169872415480783042290993786_f64;

/// 1/π
pub const FRAC_1_PI: f64 = 0.318309886183790671537767526745028724_f64;

/// 1/sqrt(π)
pub const FRAC_1_SQRT_PI: f64 = 0.564189583547756286948079451560772586_f64;

/// 2/π
pub const FRAC_2_PI: f64 = 0.636619772367581343075535053490057448_f64;

/// 2/sqrt(π)
pub const FRAC_2_SQRT_PI: f64 = 1.12837916709551257389615890312154517_f64;

/// sqrt(2)
pub const SQRT_2: f64 = 1.41421356237309504880168872420969808_f64;

/// 1/sqrt(2)
pub const FRAC_1_SQRT_2: f64 = 0.707106781186547524400844362104849039_f64;

/// sqrt(3)
pub const SQRT_3: f64 = 1.732050807568877293527446341505872367_f64;

/// 1/sqrt(3)
pub const FRAC_1_SQRT_3: f64 = 0.577350269189625764509148780501957456_f64;

/// Euler's number (e)
pub const E: f64 = 2.71828182845904523536028747135266250_f64;

/// log<sub>2</sub>(10)
pub const LOG2_10: f64 = 3.32192809488736234787031942948939018_f64;

/// log<sub>2</sub>(e)
pub const LOG2_E: Float64 = 1.44269504088896340735992468100189214_f64;

/// log<sub>10</sub>(2)
pub const LOG10_2: Float64 = 0.301029995663981195213738894724493027_f64;

/// log<sub>10</sub>(e)
pub const LOG10_E: Float64 = 0.434294481903251827651128918916605082_f64;

/// ln(2)
pub const LN_2: Float64 = 0.693147180559945309417232121458176568_f64;

/// ln(10)
pub const LN_10: Float64 = 2.30258509299404568401799145468436421_f64;

/// Archimedes' constant (π) as a 32-bit float
pub const PIF: Float32 = 3.14159265358979323846264338327950288_f32;

/// The full circle constant (τ) as a 32-bit float
///
/// Equal to 2π.
pub const TAUF: Float32 = 6.28318530717958647692528676655900577_f32;

/// The golden ratio (φ) as a 32-bit float
pub const PHIF: Float32 = 1.618033988749894848204586834365638118_f32;

/// The Euler-Mascheroni constant (γ) as a 32-bit float
pub const EGAMMAF: Float32 = 0.577215664901532860606512090082402431_f32;

/// π/2 as a 32-bit float
pub const FRAC_PI_2F: Float32 = 1.57079632679489661923132169163975144_f32;

/// π/3 as a 32-bit float
pub const FRAC_PI_3F: Float32 = 1.04719755119659774615421446109316763_f32;

/// π/4 as a 32-bit float
pub const FRAC_PI_4F: Float32 = 0.785398163397448309615660845819875721_f32;

/// π/6 as a 32-bit float
pub const FRAC_PI_6F: Float32 = 0.52359877559829887307710723054658381_f32;

/// π/8 as a 32-bit float
pub const FRAC_PI_8F: Float32 = 0.39269908169872415480783042290993786_f32;

/// 1/π as a 32-bit float
pub const FRAC_1_PIF: Float32 = 0.318309886183790671537767526745028724_f32;

/// 1/sqrt(π) as a 32-bit float
pub const FRAC_1_SQRT_PIF: Float32 = 0.564189583547756286948079451560772586_f32;

/// 2/π as a 32-bit float
pub const FRAC_2_PIF: Float32 = 0.636619772367581343075535053490057448_f32;

/// 2/sqrt(π) as a 32-bit float
pub const FRAC_2_SQRT_PIF: Float32 = 1.12837916709551257389615890312154517_f32;

/// sqrt(2) as a 32-bit float
pub const SQRT_2F: Float32 = 1.41421356237309504880168872420969808_f32;

/// 1/sqrt(2) as a 32-bit float
pub const FRAC_1_SQRT_2F: Float32 = 0.707106781186547524400844362104849039_f32;

/// sqrt(3) as a 32-bit float
pub const SQRT_3F: Float32 = 1.732050807568877293527446341505872367_f32;

/// 1/sqrt(3) as a 32-bit float
pub const FRAC_1_SQRT_3F: Float32 = 0.577350269189625764509148780501957456_f32;

/// Euler's number (e) as a 32-bit float
pub const EF: Float32 = 2.71828182845904523536028747135266250_f32;

/// log<sub>2</sub>(e) as a 32-bit float
pub const LOG2_EF: Float32 = 1.44269504088896340735992468100189214_f32;

/// log<sub>2</sub>(10) as a 32-bit float
pub const LOG2_10F: Float32 = 3.32192809488736234787031942948939018_f32;

/// log<sub>10</sub>(e) as a 32-bit float
pub const LOG10_EF: Float32 = 0.434294481903251827651128918916605082_f32;

/// log<sub>10</sub>(2) as a 32-bit float
pub const LOG10_2F: Float32 = 0.301029995663981195213738894724493027_f32;

/// ln(2) as a 32-bit float
pub const LN_2F: Float32 = 0.693147180559945309417232121458176568_f32;

/// ln(10) as a 32-bit float
pub const LN_10F: Float32 = 2.30258509299404568401799145468436421_f32;
