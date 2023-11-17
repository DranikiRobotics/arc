// // ~10 * machine epsilon
// private const val EPS = 2.2e-15

use alloc::{vec, vec::Vec};
use llm::Float64;

const EPS: Float64 = 2.2e-15;

/// Function `snz(x)` from section VI.A of the [SymForce paper](https://arxiv.org/abs/2204.07889)
/// for use in singularity handling.
fn snz(x: Float64) -> Float64 {
    if x >= 0.0 {
        EPS
    } else {
        -EPS
    }
}

fn clamp(x: Float64, lo: Float64, hi: Float64) -> Float64 {
    if x < lo {
        lo
    } else if x > hi {
        hi
    } else {
        x
    }
}

struct MinMax {
    pub min: Float64,
    pub max: Float64,
}

/// Partitions `[a, b]` into `n` equal intervals and returns the center values.
fn range(begin: Float64, end: Float64, samples: usize) -> Vec<Float64> {
    assert!(samples >= 2);
    let dx = (end - begin) / (samples - 1) as Float64;
    (0..samples).map(|i| begin + dx * i as Float64).collect()
}

/// Partitions `[a, b]` into `n` equal intervals and returns the center values.
fn range_centered(begin: Float64, end: Float64, samples: usize) -> Vec<Float64> {
    assert!(samples >= 1);
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
        i *= eps / llm::ulp(1.0);

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

#[test]
fn test_integral_scan() {
    let f = |x: Float64| x * x;
    let scan = IntegralScanResult::scan(0.0, 1.0, 1e-6, f);
    panic!("{:?} {:?}", scan.sums, scan.values);
}

fn binsearch(values: &[Float64], query: &Float64) -> Result<usize, usize> {
    let mut size = values.len();
    let mut left = 0;
    let mut right = size;
    while left < right {
        let mid = left + size / 2;

        // SAFETY: the while condition means `size` is strictly positive, so
        // `size/2 < size`. Thus `left + size/2 < left + size`, which
        // coupled with the `left + size <= self.len()` invariant means
        // we have `left + size/2 < self.len()`, and this is in-bounds.
        let cmp = unsafe {
            values
                .get_unchecked(mid)
                .partial_cmp(&query)
                .unwrap_or_else(|| core::hint::unreachable_unchecked())
        };

        // The reason why we use if/else control flow rather than match
        // is because match reorders comparison operations, which is perf sensitive.
        // This is x86 asm for u8: https://rust.godbolt.org/z/8Y8Pra.
        if cmp == core::cmp::Ordering::Less {
            left = mid + 1;
        } else if cmp == core::cmp::Ordering::Greater {
            right = mid;
        } else {
            // SAFETY: same as the `get_unchecked` above
            unsafe { core::intrinsics::assume(mid < values.len()) };
            return Ok(mid);
        }

        size = right - left;
    }

    // SAFETY: directly true from the overall invariant.
    // Note that this is `<=`, unlike the assume in the `Ok` path.
    unsafe { core::intrinsics::assume(left <= values.len()) };
    Err(left)
}

fn lerpLookup(source: &[Float64], target: &[Float64], query: Float64) -> Float64 {
    assert_eq!(source.len(), target.len());
    assert!(!source.is_empty());

    let index = binsearch(source, &query).unwrap_or_else(|i| i);
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

// // precondition: source, target sorted and share the same length
// fun lerpLookup(source: List<Double>, target: List<Double>, query: Double): Double {
//     require(source.size == target.size)
//     require(source.isNotEmpty())
//
//     val index = source.binarySearch(query)
//     return if (index >= 0) {
//         target[index]
//     } else {
//         val insIndex = -(index + 1)
//         when {
//             insIndex <= 0 -> target.first()
//             insIndex >= source.size -> target.last()
//             else -> {
//                 val sLo = source[insIndex - 1]
//                 val sHi = source[insIndex]
//                 val tLo = target[insIndex - 1]
//                 val tHi = target[insIndex]
//                 lerp(query, sLo, sHi, tLo, tHi)
//             }
//         }
//     }
// }
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
