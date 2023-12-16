#![cfg(test)]

#[allow(unused)]
use crate::u2;

#[test]
fn test_partial_cmp() {
    assert!(u2::ZERO == u2::ZERO);
    assert!(u2::ZERO < u2::ONE);
    assert!(u2::ZERO < u2::TWO);
    assert!(u2::ZERO < u2::THREE);
    assert!(u2::ONE > u2::ZERO);
    assert!(u2::ONE == u2::ONE);
    assert!(u2::ONE < u2::TWO);
    assert!(u2::ONE < u2::THREE);
    assert!(u2::TWO > u2::ZERO);
    assert!(u2::TWO > u2::ONE);
    assert!(u2::TWO == u2::TWO);
    assert!(u2::TWO < u2::THREE);
    assert!(u2::THREE > u2::ZERO);
    assert!(u2::THREE > u2::ONE);
    assert!(u2::THREE > u2::TWO);
    assert!(u2::THREE == u2::THREE);
}

#[test]
fn test_add() {
    assert_eq!(u2::ZERO + u2::ZERO, u2::ZERO);
    assert_eq!(u2::ZERO + u2::ONE, u2::ONE);
    assert_eq!(u2::ZERO + u2::TWO, u2::TWO);
    assert_eq!(u2::ZERO + u2::THREE, u2::THREE);
    assert_eq!(u2::ONE + u2::ZERO, u2::ONE);
    assert_eq!(u2::ONE + u2::ONE, u2::TWO);
    assert_eq!(u2::ONE + u2::TWO, u2::THREE);
    assert_eq!(u2::TWO + u2::ZERO, u2::TWO);
    assert_eq!(u2::TWO + u2::ONE, u2::THREE);
    assert_eq!(u2::THREE + u2::ZERO, u2::THREE);
}

#[test]
fn test_sub() {
    assert_eq!(u2::ZERO - u2::ZERO, u2::ZERO);
    assert_eq!(u2::ONE - u2::ZERO, u2::ONE);
    assert_eq!(u2::ONE - u2::ONE, u2::ZERO);
    assert_eq!(u2::TWO - u2::ZERO, u2::TWO);
    assert_eq!(u2::TWO - u2::ONE, u2::ONE);
    assert_eq!(u2::TWO - u2::TWO, u2::ZERO);
    assert_eq!(u2::THREE - u2::ZERO, u2::THREE);
    assert_eq!(u2::THREE - u2::ONE, u2::TWO);
    assert_eq!(u2::THREE - u2::TWO, u2::ONE);
    assert_eq!(u2::THREE - u2::THREE, u2::ZERO);
}

#[test]
fn test_mul() {
    assert_eq!(u2::ZERO * u2::ZERO, u2::ZERO);
    assert_eq!(u2::ZERO * u2::ONE, u2::ZERO);
    assert_eq!(u2::ZERO * u2::TWO, u2::ZERO);
    assert_eq!(u2::ZERO * u2::THREE, u2::ZERO);
    assert_eq!(u2::ONE * u2::ZERO, u2::ZERO);
    assert_eq!(u2::ONE * u2::ONE, u2::ONE);
    assert_eq!(u2::ONE * u2::TWO, u2::TWO);
    assert_eq!(u2::TWO * u2::ZERO, u2::ZERO);
    assert_eq!(u2::TWO * u2::ONE, u2::TWO);
    assert_eq!(u2::THREE * u2::ZERO, u2::ZERO);
    assert_eq!(u2::THREE * u2::ONE, u2::THREE);
}

#[test]
fn test_div() {
    assert_eq!(u2::ZERO / u2::ONE, u2::ZERO);
    assert_eq!(u2::ZERO / u2::TWO, u2::ZERO);
    assert_eq!(u2::ZERO / u2::THREE, u2::ZERO);
    assert_eq!(u2::ONE / u2::ONE, u2::ONE);
    assert_eq!(u2::TWO / u2::ONE, u2::TWO);
    assert_eq!(u2::TWO / u2::TWO, u2::ONE);
    assert_eq!(u2::THREE / u2::ONE, u2::THREE);
    assert_eq!(u2::THREE / u2::THREE, u2::ONE);
}

#[test]
fn from_u2() {
    assert_eq!(u8::from(u2::ZERO), 0);
    assert_eq!(u8::from(u2::ONE), 1);
    assert_eq!(u8::from(u2::TWO), 2);
    assert_eq!(u8::from(u2::THREE), 3);
}

#[test]
fn from_u8() {
    assert_eq!(u2::from(0), u2::ZERO);
    assert_eq!(u2::from(1), u2::ONE);
    assert_eq!(u2::from(2), u2::TWO);
    assert_eq!(u2::from(3), u2::THREE);
}
