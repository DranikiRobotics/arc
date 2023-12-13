/* origin: FreeBSD /usr/src/lib/msun/src/s_fmaf.c */
/*-
 * Copyright (c) 2005-2011 David Schultz <das@FreeBSD.ORG>
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE AUTHOR AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
*/

use crate::{Float64, Float32, Int};

use core::ptr::read_volatile;

use super::fenv::{
    feclearexcept, fegetround, feraiseexcept, fetestexcept, FE_INEXACT, FE_TONEAREST, FE_UNDERFLOW,
};

/**
 * Fused multiply-add: Compute x * y + z with a single rounding error.
 *
 * A double has more than twice as much precision than a float, so
 * direct double-precision arithmetic suffices, except where double
 * rounding occurs.
*/

/// Floating multiply add
///
/// Computes `(x*y)+z`, rounded as one ternary operation:
/// Computes the value (as if) to infinite precision and rounds once to the result format,
/// according to the rounding mode characterized by the value of FLT_ROUNDS.
#[export_name = "__l2math_fmaf"]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub extern "C" fn fmaf(x: Float32, y: Float32, mut z: Float32) -> Float32 {
    let mut result: Float64;
    let mut ui: u64;
    let xy: Float64 = x as Float64 * y as Float64;
    result = xy + z as Float64;
    ui = result.to_bits();
    let e: Int = (ui >> 52) as Int & 0x7ff;
    /* Common case: The double precision result is fine. */
    if (
        /* not a halfway case */
        ui & 0x1fffffff) != 0x10000000 ||
        /* NaN */
        e == 0x7ff ||
        /* exact */
        (result - xy == z as Float64 && result - z as Float64 == xy) ||
        /* not round-to-nearest */
        fegetround() != FE_TONEAREST
    {
        /*
            underflow may not be raised correctly, example:
            fmaf(0x1p-120f, 0x1p-120f, 0x1p-149f)
        */
        if (0x3ff - 149..0x3ff - 126).contains(&e) && fetestexcept(FE_INEXACT) != 0 {
            feclearexcept(FE_INEXACT);
            // prevent `xy + vz` from being CSE'd with `xy + z` above
            let vz: Float32 = unsafe { read_volatile(&z) };
            result = xy + vz as Float64;
            if fetestexcept(FE_INEXACT) != 0 {
                feraiseexcept(FE_UNDERFLOW);
            } else {
                feraiseexcept(FE_INEXACT);
            }
        }
        z = result as Float32;
        return z;
    }

    // If result is inexact, and exactly halfway between two float values,
    // we need to adjust the low-order bit in the direction of the error.
    let neg = ui >> 63 != 0;
    let err = if neg == (z as Float64 > xy) {
        xy - result + z as Float64
    } else {
        z as Float64 - result + xy
    };
    if neg == (err < 0.0) {
        ui += 1;
    } else {
        ui -= 1;
    }
    Float64::from_bits(ui) as Float32
}

#[cfg(test)]
mod tests {
    #[test]
    fn issue_263() {
        let a = f32::from_bits(1266679807);
        let b = f32::from_bits(1300234242);
        let c = f32::from_bits(1115553792);
        let expected = f32::from_bits(1501560833);
        assert_eq!(super::fmaf(a, b, c), expected);
    }
}
