use crate::{Float32, Int};

/// Return the remainder and part of the quotient of `x` and `y`.
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn remquof(mut x: Float32, mut y: Float32) -> (Float32, Int) {
    let ux: u32 = x.to_bits();
    let mut uy: u32 = y.to_bits();
    let mut ex = ((ux >> 23) & 0xff) as Int;
    let mut ey = ((uy >> 23) & 0xff) as Int;
    let sx = (ux >> 31) != 0;
    let sy = (uy >> 31) != 0;
    let mut q: u32;
    let mut i: u32;
    let mut uxi: u32 = ux;

    if (uy << 1) == 0 || y.is_nan() || ex == 0xff {
        return ((x * y) / (x * y), 0);
    }
    if (ux << 1) == 0 {
        return (x, 0);
    }

    /* normalize x and y */
    if ex == 0 {
        i = uxi << 9;
        while (i >> 31) == 0 {
            ex -= 1;
            i <<= 1;
        }
        uxi <<= -ex + 1;
    } else {
        uxi &= (!0) >> 9;
        uxi |= 1 << 23;
    }
    if ey == 0 {
        i = uy << 9;
        while (i >> 31) == 0 {
            ey -= 1;
            i <<= 1;
        }
        uy <<= -ey + 1;
    } else {
        uy &= (!0) >> 9;
        uy |= 1 << 23;
    }

    q = 0;
    if ex + 1 != ey {
        if ex < ey {
            return (x, 0);
        }
        /* x mod y */
        while ex > ey {
            i = uxi.wrapping_sub(uy);
            if (i >> 31) == 0 {
                uxi = i;
                q += 1;
            }
            uxi <<= 1;
            q <<= 1;
            ex -= 1;
        }
        i = uxi.wrapping_sub(uy);
        if (i >> 31) == 0 {
            uxi = i;
            q += 1;
        }
        if uxi == 0 {
            ex = -30;
        } else {
            while (uxi >> 23) == 0 {
                uxi <<= 1;
                ex -= 1;
            }
        }
    }

    /* scale result and decide between |x| and |x|-|y| */
    if ex > 0 {
        uxi -= 1 << 23;
        uxi |= (ex as u32) << 23;
    } else {
        uxi >>= -ex + 1;
    }
    x = Float32::from_bits(uxi);
    if sy {
        y = -y;
    }
    if ex == ey || (ex + 1 == ey && (2.0 * x > y || (2.0 * x == y && (q % 2) != 0))) {
        x -= y;
        q += 1;
    }
    q &= 0x7fffffff;
    let quo = if sx ^ sy { -(q as Int) } else { q as Int };
    if sx {
        (-x, quo)
    } else {
        (x, quo)
    }
}

/// FFI bindings for remquof
#[inline]
#[doc(hidden)]
#[export_name = "__l2math_remquof"]
pub extern "C" fn __l2math_remquof(x: Float32, y: Float32) -> super::Tuple_Float32_Int {
    super::Tuple_Float32_Int::from(remquof(x, y))
}
