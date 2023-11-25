#[path = "./u2/mod.rs"] mod _u2;
#[path = "./u3/mod.rs"] mod _u3;

pub use _u2::u2;
pub use _u3::u3;

impl From<u2> for u3 {
    #[must_use]
    #[inline(always)]
    fn from(u: u2) -> Self {
        u3::new(u.0, u.1, false)
    }
}

impl From<u3> for u2 {
    #[must_use]
    #[inline(always)]
    fn from(u: u3) -> Self {
        u2::new(u.0, u.1)
    }
}
