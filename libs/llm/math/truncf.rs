use crate::{Float32, Int};

/// Returns the integer part of self.
/// This means that non-integer numbers are always truncated towards zero.
#[export_name = "__llm_truncf"]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub extern "C" fn truncf(x: Float32) -> Float32 {
    // On wasm32 we know that LLVM's intrinsic will compile to an optimized
    // `Float32.trunc` native instruction, so we can leverage this for both code size
    // and speed.
    llvm_intrinsically_optimized! {
        #[cfg(target_arch = "wasm32")] {
            return unsafe { ::core::intrinsics::truncf32(x) }
        }
    }
    let x1p120 = Float32::from_bits(0x7b800000); // 0x1p120f === 2 ^ 120

    let mut i: u32 = x.to_bits();
    let mut e: Int = (i >> 23 & 0xff) as Int - 0x7f + 9;

    if e >= 23 + 9 {
        return x;
    }
    if e < 9 {
        e = 1;
    }
    let m: u32 = -1i32 as u32 >> e;
    if (i & m) == 0 {
        return x;
    }
    force_eval!(x + x1p120);
    i &= !m;
    Float32::from_bits(i)
}

// PowerPC tests are failing on LLVM 13: https://github.com/rust-lang/rust/issues/88520
#[cfg(not(target_arch = "powerpc64"))]
#[cfg(test)]
mod tests {
    #[test]
    fn sanity_check() {
        assert_eq!(super::truncf(1.1), 1.0);
    }
}
