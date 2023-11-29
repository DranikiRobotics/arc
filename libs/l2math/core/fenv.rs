// src: musl/src/fenv/fenv.c
use crate::Int;
/* Dummy functions for archs lacking fenv implementation */

pub(crate) const FE_UNDERFLOW: Int = 0;
pub(crate) const FE_INEXACT: Int = 0;

pub(crate) const FE_TONEAREST: Int = 0;

#[inline]
pub(crate) fn feclearexcept(_mask: Int) -> Int {
    0
}

#[inline]
pub(crate) fn feraiseexcept(_mask: Int) -> Int {
    0
}

#[inline]
pub(crate) fn fetestexcept(_mask: Int) -> Int {
    0
}

#[inline]
pub(crate) fn fegetround() -> Int {
    FE_TONEAREST
}
