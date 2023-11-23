// // ~10 * machine epsilon
// private const val EPS = 2.2e-15

use alloc::{vec, vec::Vec};
use l2math::Float64;

const EPS: Float64 = 2.2e-15;

/// Function `snz(x)` from section VI.A of the [SymForce paper](https://arxiv.org/abs/2204.07889)
/// for use in singularity handling.
fn snz(x: Float64) -> Float64 {
    if x >= 0.0 {
        return EPS;
    }
    -EPS
}

fn clamp(x: Float64, lo: Float64, hi: Float64) -> Float64 {
    if x < lo {
        return lo;
    }
    if x > hi {
        return hi;
    }
    x
}

struct MinMax {
    pub min: Float64,
    pub max: Float64,
}

/// Partitions `[a, b]` into `n` equal intervals and returns the center values.
fn range(begin: Float64, end: Float64, samples: usize) -> Vec<Float64> {
    debug_assert!(samples >= 2);
    let dx = (end - begin) / (samples - 1) as Float64;
    (0..samples).map(|i| begin + dx * i as Float64).collect()
}

/// Partitions `[a, b]` into `n` equal intervals and returns the center values.
fn range_centered(begin: Float64, end: Float64, samples: usize) -> Vec<Float64> {
    debug_assert!(samples >= 1);
    let dx = (end - begin) / samples as Float64;
    (0..samples)
        .map(|i| begin + 0.5 * dx + dx * i as Float64)
        .collect()
}

fn lerp(x: Float64, from_lo: Float64, from_hi: Float64, to_lo: Float64, to_hi: Float64) -> Float64 {
    if from_lo == from_hi {
        return 0.0;
    }
    to_lo + (x - from_lo) * (to_hi - to_lo) / (from_hi - from_lo)
}

pub struct IntegralScanResult<F: Fn(Float64) -> Float64> {
    pub values: Vec<Float64>,
    pub sums: Vec<Float64>,
    __func: Option<F>,
    __i: Float64,
}

impl<F: (Fn(Float64) -> Float64) + Clone> IntegralScanResult<F> {
    /// Returns samples of `g(t) = ∫a^t f(x) dx` for various values `a ≤ t ≤ b`.
    /// The sampling points are chosen adaptively using the algorithm `adaptsim` from
    /// [Gander and Gautschi](https://doi.org/10.1023/A:1022318402393)
    /// ([more accessible link](https://users.wpi.edu/~walker/MA510/HANDOUTS/w.gander,w.gautschi,Adaptive_Quadrature,BIT_40,2000,84-101.pdf)).
    pub fn scan(a: Float64, b: Float64, eps: Float64, f: F) -> Self {
        let mut this = Self {
            values: vec![0.0],
            sums: vec![0.0],
            __func: None,
            __i: 0.0,
        };

        let m = (a + b) / 2.0;
        let fa = f(a);
        let fm = f(m);
        let fb = f(b);

        let mut i = (b - a) / 8.0
            * (fa
                + fm
                + fb
                + f(a + 0.9501 * (b - a))
                + f(a + 0.2311 * (b - a))
                + f(a + 0.6068 * (b - a))
                + f(a + 0.4860 * (b - a))
                + f(a + 0.8913 * (b - a)));
        if i == 0.0 {
            i = b - a;
        }
        i *= eps / l2math::ulp(1.0);

        this.__func = Some(f);
        this.__i = i;

        this.helper(a, m, b, fa, fm, fb);

        this
    }
    fn helper(
        &mut self,
        a: Float64,
        m: Float64,
        b: Float64,
        fa: Float64,
        fm: Float64,
        fb: Float64,
    ) {
        let h = (b - a) / 4.0;
        let ml = a + h;
        let mr = b - h;
        let fml = (self.__func.as_ref().unwrap())(ml);
        let fmr = (self.__func.as_ref().unwrap())(mr);
        let mut i1 = h / 1.5 * (fa + 4.0 * fm + fb);
        let i2 = h / 3.0 * (fa + 4.0 * (fml + fmr) + 2.0 * fm + fb);
        i1 = (16.0 * i2 - i1) / 15.0;
        if self.__i + (i1 - i2) == self.__i || m <= a || b <= m {
            self.values.push(b);
            self.sums.push(self.sums.last().unwrap() + i1);
        } else {
            self.helper(a, ml, m, fa, fml, fm);
            self.helper(m, mr, b, fm, fmr, fb);
        }
    }
}

/// Linearly interpolates `query` between `source` and `target`.
///
/// precondition: source, target sorted and share the same length
fn lerpLookup(source: &[Float64], target: &[Float64], query: Float64) -> Float64 {
    debug_assert_eq!(source.len(), target.len());
    debug_assert!(!source.is_empty());

    let index = source
        .binary_search_by(|&x| x.partial_cmp(&query).unwrap())
        .unwrap_or_else(|i| i);
    if index < source.len() {
        target[index]
    } else {
        let s_lo = source[index - 1];
        let s_hi = source[index];
        let t_lo = target[index - 1];
        let t_hi = target[index];
        lerp(query, s_lo, s_hi, t_lo, t_hi)
    }
}

/// Linearly interpolates `queries` between `source` and `target`.
///
/// precondition: source, target sorted and share the same length; queries sorted
fn lerpLookupMap(source: &[Float64], target: &[Float64], queries: &[Float64]) -> Vec<Float64> {
    debug_assert_eq!(source.len(), target.len());
    debug_assert!(!source.is_empty());

    let mut result = Vec::with_capacity(queries.len());

    let mut i = 0;
    for &query in queries {
        if query < source[0] {
            result.push(target[0]);
            continue;
        }

        while i + 1 < source.len() && source[i + 1] < query {
            i += 1;
        }

        if i + 1 == source.len() {
            result.push(target.last().copied().unwrap());
            continue;
        }

        let s_lo = source[i];
        let s_hi = source[i + 1];
        let t_lo = target[i];
        let t_hi = target[i + 1];
        result.push(lerp(query, s_lo, s_hi, t_lo, t_hi));
    }

    result
}

//
// // precondition: source, target sorted and share the same length; queries sorted
// fun lerpLookupMap(source: List<Double>, target: List<Double>, queries: List<Double>): List<Double> {
//     require(source.size == target.size)
//     require(source.isNotEmpty())
//
//     val result = mutableListOf<Double>()
//
//     var i = 0
//     for (query in queries) {
//         if (query < source[0]) {
//             result.add(target[0])
//             continue
//         }
//
//         while (i + 1 < source.size && source[i + 1] < query) {
//             i++
//         }
//
//         if (i + 1 == source.size) {
//             result.add(target.last())
//             continue
//         }
//
//         val sLo = source[i]
//         val sHi = source[i + 1]
//         val tLo = target[i]
//         val tHi = target[i + 1]
//         result.add(lerp(query, sLo, sHi, tLo, tHi))
//     }
//
//     return result
// }
