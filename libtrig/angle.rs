use crate::*;

/// A wrapper around a `Float` value that represents an angle.
///
/// It can be created from either radians or degrees, and can be converted to either radians or degrees.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Angle(Float, bool);

impl Angle {
    /// Creates a new `Angle` from the given `value` and `is_radians` flag.
    #[inline]
    #[must_use]
    pub const fn new(value: Float, is_radians: bool) -> Self {
        Self(value, is_radians)
    }
    /// Creates a new `Angle` from the given `value` in radians.
    #[inline]
    #[must_use]
    pub const fn from_radians(value: Float) -> Self {
        Self::new(value, true)
    }
    /// Creates a new `Angle` from the given `value` in degrees.
    #[inline]
    #[must_use]
    pub const fn from_degrees(value: Float) -> Self {
        Self::new(value, false)
    }
    /// Returns the value of the angle in radians.
    #[inline]
    #[must_use]
    pub fn to_radians(&self) -> Float {
        if self.1 {
            self.0
        } else {
            self.0.to_radians()
        }
    }
    /// Returns the value of the angle in degrees.
    #[inline]
    #[must_use]
    pub fn to_degrees(&self) -> Float {
        if self.1 {
            self.0.to_degrees()
        } else {
            self.0
        }
    }
}

impl std::fmt::Display for Angle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}°", self.to_degrees())
    }
}
