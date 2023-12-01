#[path = "./u2/mod.rs"]
mod _u2;
#[path = "./u3/mod.rs"]
mod _u3;

/// A simple type alias for a bit.
#[allow(non_camel_case_types)]
pub type bit = bool;

pub use _u2::u2;
pub use _u3::u3;

impl From<u2> for u3 {
    #[must_use]
    #[inline(always)]
    fn from(u: u2) -> Self {
        u3::fromu2(u)
    }
}

impl From<u3> for u2 {
    #[must_use]
    #[inline(always)]
    fn from(u: u3) -> Self {
        u2::fromu3(u)
    }
}
