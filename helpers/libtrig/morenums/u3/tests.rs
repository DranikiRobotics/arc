#![cfg(test)]

#[allow(unused)]
use crate::u3;

#[test]
fn test_partial_cmp() {
    assert_eq!(
        u3::ZERO.partial_cmp(&u3::ZERO),
        Some(core::cmp::Ordering::Equal)
    );
    assert_eq!(
        u3::ZERO.partial_cmp(&u3::ONE),
        Some(core::cmp::Ordering::Less)
    );
    assert_eq!(
        u3::ZERO.partial_cmp(&u3::TWO),
        Some(core::cmp::Ordering::Less)
    );
    assert_eq!(
        u3::ZERO.partial_cmp(&u3::THREE),
        Some(core::cmp::Ordering::Less)
    );
    assert_eq!(
        u3::ZERO.partial_cmp(&u3::FOUR),
        Some(core::cmp::Ordering::Less)
    );
    assert_eq!(
        u3::ZERO.partial_cmp(&u3::FIVE),
        Some(core::cmp::Ordering::Less)
    );
    assert_eq!(
        u3::ZERO.partial_cmp(&u3::SIX),
        Some(core::cmp::Ordering::Less)
    );
    assert_eq!(
        u3::ZERO.partial_cmp(&u3::SEVEN),
        Some(core::cmp::Ordering::Less)
    );
    assert_eq!(
        u3::ONE.partial_cmp(&u3::ZERO),
        Some(core::cmp::Ordering::Greater)
    );
    assert_eq!(
        u3::ONE.partial_cmp(&u3::ONE),
        Some(core::cmp::Ordering::Equal)
    );
    assert_eq!(
        u3::ONE.partial_cmp(&u3::TWO),
        Some(core::cmp::Ordering::Less)
    );
    assert_eq!(
        u3::ONE.partial_cmp(&u3::THREE),
        Some(core::cmp::Ordering::Less)
    );
    assert_eq!(
        u3::ONE.partial_cmp(&u3::FOUR),
        Some(core::cmp::Ordering::Less)
    );
    assert_eq!(
        u3::ONE.partial_cmp(&u3::FIVE),
        Some(core::cmp::Ordering::Less)
    );
    assert_eq!(
        u3::ONE.partial_cmp(&u3::SIX),
        Some(core::cmp::Ordering::Less)
    );
    assert_eq!(
        u3::ONE.partial_cmp(&u3::SEVEN),
        Some(core::cmp::Ordering::Less)
    );
    assert_eq!(
        u3::TWO.partial_cmp(&u3::ZERO),
        Some(core::cmp::Ordering::Greater)
    );
    assert_eq!(
        u3::TWO.partial_cmp(&u3::ONE),
        Some(core::cmp::Ordering::Greater)
    );
    assert_eq!(
        u3::TWO.partial_cmp(&u3::TWO),
        Some(core::cmp::Ordering::Equal)
    );
    assert_eq!(
        u3::TWO.partial_cmp(&u3::THREE),
        Some(core::cmp::Ordering::Less)
    );
    assert_eq!(
        u3::TWO.partial_cmp(&u3::FOUR),
        Some(core::cmp::Ordering::Less)
    );
    assert_eq!(
        u3::TWO.partial_cmp(&u3::FIVE),
        Some(core::cmp::Ordering::Less)
    );
    assert_eq!(
        u3::TWO.partial_cmp(&u3::SIX),
        Some(core::cmp::Ordering::Less)
    );
    assert_eq!(
        u3::TWO.partial_cmp(&u3::SEVEN),
        Some(core::cmp::Ordering::Less)
    );
    assert_eq!(
        u3::THREE.partial_cmp(&u3::ZERO),
        Some(core::cmp::Ordering::Greater)
    );
    assert_eq!(
        u3::THREE.partial_cmp(&u3::ONE),
        Some(core::cmp::Ordering::Greater)
    );
    assert_eq!(
        u3::THREE.partial_cmp(&u3::TWO),
        Some(core::cmp::Ordering::Greater)
    );
    assert_eq!(
        u3::THREE.partial_cmp(&u3::THREE),
        Some(core::cmp::Ordering::Equal)
    );
    assert_eq!(
        u3::THREE.partial_cmp(&u3::FOUR),
        Some(core::cmp::Ordering::Less)
    );
    assert_eq!(
        u3::THREE.partial_cmp(&u3::FIVE),
        Some(core::cmp::Ordering::Less)
    );
    assert_eq!(
        u3::THREE.partial_cmp(&u3::SIX),
        Some(core::cmp::Ordering::Less)
    );
    assert_eq!(
        u3::THREE.partial_cmp(&u3::SEVEN),
        Some(core::cmp::Ordering::Less)
    );
    assert_eq!(
        u3::FOUR.partial_cmp(&u3::ZERO),
        Some(core::cmp::Ordering::Greater)
    );
    assert_eq!(
        u3::FOUR.partial_cmp(&u3::ONE),
        Some(core::cmp::Ordering::Greater)
    );
    assert_eq!(
        u3::FOUR.partial_cmp(&u3::TWO),
        Some(core::cmp::Ordering::Greater)
    );
    assert_eq!(
        u3::FOUR.partial_cmp(&u3::THREE),
        Some(core::cmp::Ordering::Greater)
    );
    assert_eq!(
        u3::FOUR.partial_cmp(&u3::FOUR),
        Some(core::cmp::Ordering::Equal)
    );
    assert_eq!(
        u3::FOUR.partial_cmp(&u3::FIVE),
        Some(core::cmp::Ordering::Less)
    );
    assert_eq!(
        u3::FOUR.partial_cmp(&u3::SIX),
        Some(core::cmp::Ordering::Less)
    );
    assert_eq!(
        u3::FOUR.partial_cmp(&u3::SEVEN),
        Some(core::cmp::Ordering::Less)
    );
    assert_eq!(
        u3::FIVE.partial_cmp(&u3::ZERO),
        Some(core::cmp::Ordering::Greater)
    );
    assert_eq!(
        u3::FIVE.partial_cmp(&u3::ONE),
        Some(core::cmp::Ordering::Greater)
    );
    assert_eq!(
        u3::FIVE.partial_cmp(&u3::TWO),
        Some(core::cmp::Ordering::Greater)
    );
    assert_eq!(
        u3::FIVE.partial_cmp(&u3::THREE),
        Some(core::cmp::Ordering::Greater)
    );
    assert_eq!(
        u3::FIVE.partial_cmp(&u3::FOUR),
        Some(core::cmp::Ordering::Greater)
    );
    assert_eq!(
        u3::FIVE.partial_cmp(&u3::FIVE),
        Some(core::cmp::Ordering::Equal)
    );
    assert_eq!(
        u3::FIVE.partial_cmp(&u3::SIX),
        Some(core::cmp::Ordering::Less)
    );
    assert_eq!(
        u3::FIVE.partial_cmp(&u3::SEVEN),
        Some(core::cmp::Ordering::Less)
    );
    assert_eq!(
        u3::SIX.partial_cmp(&u3::ZERO),
        Some(core::cmp::Ordering::Greater)
    );
    assert_eq!(
        u3::SIX.partial_cmp(&u3::ONE),
        Some(core::cmp::Ordering::Greater)
    );
    assert_eq!(
        u3::SIX.partial_cmp(&u3::TWO),
        Some(core::cmp::Ordering::Greater)
    );
    assert_eq!(
        u3::SIX.partial_cmp(&u3::THREE),
        Some(core::cmp::Ordering::Greater)
    );
    assert_eq!(
        u3::SIX.partial_cmp(&u3::FOUR),
        Some(core::cmp::Ordering::Greater)
    );
    assert_eq!(
        u3::SIX.partial_cmp(&u3::FIVE),
        Some(core::cmp::Ordering::Greater)
    );
    assert_eq!(
        u3::SIX.partial_cmp(&u3::SIX),
        Some(core::cmp::Ordering::Equal)
    );
    assert_eq!(
        u3::SIX.partial_cmp(&u3::SEVEN),
        Some(core::cmp::Ordering::Less)
    );
    assert_eq!(
        u3::SEVEN.partial_cmp(&u3::ZERO),
        Some(core::cmp::Ordering::Greater)
    );
    assert_eq!(
        u3::SEVEN.partial_cmp(&u3::ONE),
        Some(core::cmp::Ordering::Greater)
    );
    assert_eq!(
        u3::SEVEN.partial_cmp(&u3::TWO),
        Some(core::cmp::Ordering::Greater)
    );
    assert_eq!(
        u3::SEVEN.partial_cmp(&u3::THREE),
        Some(core::cmp::Ordering::Greater)
    );
    assert_eq!(
        u3::SEVEN.partial_cmp(&u3::FOUR),
        Some(core::cmp::Ordering::Greater)
    );
    assert_eq!(
        u3::SEVEN.partial_cmp(&u3::FIVE),
        Some(core::cmp::Ordering::Greater)
    );
    assert_eq!(
        u3::SEVEN.partial_cmp(&u3::SIX),
        Some(core::cmp::Ordering::Greater)
    );
    assert_eq!(
        u3::SEVEN.partial_cmp(&u3::SEVEN),
        Some(core::cmp::Ordering::Equal)
    );
}

#[test]
fn test_add() {
    assert_eq!(u3::ZERO + u3::ZERO, u3::ZERO);
    assert_eq!(u3::ONE + u3::ZERO, u3::ONE);
    assert_eq!(u3::ONE + u3::ONE, u3::TWO);
    assert_eq!(u3::ONE + u3::TWO, u3::THREE);
    assert_eq!(u3::ONE + u3::THREE, u3::FOUR);
    assert_eq!(u3::ONE + u3::FOUR, u3::FIVE);
    assert_eq!(u3::ONE + u3::FIVE, u3::SIX);
    assert_eq!(u3::ONE + u3::SIX, u3::SEVEN);
    assert_eq!(u3::TWO + u3::ZERO, u3::TWO);
    assert_eq!(u3::TWO + u3::ONE, u3::THREE);
    assert_eq!(u3::TWO + u3::TWO, u3::FOUR);
    assert_eq!(u3::TWO + u3::THREE, u3::FIVE);
    assert_eq!(u3::TWO + u3::FOUR, u3::SIX);
    assert_eq!(u3::TWO + u3::FIVE, u3::SEVEN);
    assert_eq!(u3::THREE + u3::ZERO, u3::THREE);
    assert_eq!(u3::THREE + u3::ONE, u3::FOUR);
    assert_eq!(u3::THREE + u3::TWO, u3::FIVE);
    assert_eq!(u3::THREE + u3::THREE, u3::SIX);
    assert_eq!(u3::THREE + u3::FOUR, u3::SEVEN);
    assert_eq!(u3::FOUR + u3::ZERO, u3::FOUR);
    assert_eq!(u3::FOUR + u3::ONE, u3::FIVE);
    assert_eq!(u3::FOUR + u3::TWO, u3::SIX);
    assert_eq!(u3::FOUR + u3::THREE, u3::SEVEN);
    assert_eq!(u3::FIVE + u3::ZERO, u3::FIVE);
    assert_eq!(u3::FIVE + u3::ONE, u3::SIX);
    assert_eq!(u3::FIVE + u3::TWO, u3::SEVEN);
    assert_eq!(u3::SIX + u3::ZERO, u3::SIX);
    assert_eq!(u3::SIX + u3::ONE, u3::SEVEN);
    assert_eq!(u3::SEVEN + u3::ZERO, u3::SEVEN);
}

#[test]
fn test_sub() {
    assert_eq!(u3::ZERO - u3::ZERO, u3::ZERO);
    assert_eq!(u3::ONE - u3::ZERO, u3::ONE);
    assert_eq!(u3::ONE - u3::ONE, u3::ZERO);
    assert_eq!(u3::TWO - u3::ZERO, u3::TWO);
    assert_eq!(u3::TWO - u3::ONE, u3::ONE);
    assert_eq!(u3::TWO - u3::TWO, u3::ZERO);
    assert_eq!(u3::THREE - u3::ZERO, u3::THREE);
    assert_eq!(u3::THREE - u3::ONE, u3::TWO);
    assert_eq!(u3::THREE - u3::TWO, u3::ONE);
    assert_eq!(u3::THREE - u3::THREE, u3::ZERO);
    assert_eq!(u3::FOUR - u3::ZERO, u3::FOUR);
    assert_eq!(u3::FOUR - u3::ONE, u3::THREE);
    assert_eq!(u3::FOUR - u3::TWO, u3::TWO);
    assert_eq!(u3::FOUR - u3::THREE, u3::ONE);
    assert_eq!(u3::FOUR - u3::FOUR, u3::ZERO);
    assert_eq!(u3::FIVE - u3::ZERO, u3::FIVE);
    assert_eq!(u3::FIVE - u3::ONE, u3::FOUR);
    assert_eq!(u3::FIVE - u3::TWO, u3::THREE);
    assert_eq!(u3::FIVE - u3::THREE, u3::TWO);
    assert_eq!(u3::FIVE - u3::FOUR, u3::ONE);
    assert_eq!(u3::FIVE - u3::FIVE, u3::ZERO);
    assert_eq!(u3::SIX - u3::ZERO, u3::SIX);
    assert_eq!(u3::SIX - u3::ONE, u3::FIVE);
    assert_eq!(u3::SIX - u3::TWO, u3::FOUR);
    assert_eq!(u3::SIX - u3::THREE, u3::THREE);
    assert_eq!(u3::SIX - u3::FOUR, u3::TWO);
    assert_eq!(u3::SIX - u3::FIVE, u3::ONE);
    assert_eq!(u3::SIX - u3::SIX, u3::ZERO);
    assert_eq!(u3::SEVEN - u3::ZERO, u3::SEVEN);
    assert_eq!(u3::SEVEN - u3::ONE, u3::SIX);
    assert_eq!(u3::SEVEN - u3::TWO, u3::FIVE);
    assert_eq!(u3::SEVEN - u3::THREE, u3::FOUR);
    assert_eq!(u3::SEVEN - u3::FOUR, u3::THREE);
    assert_eq!(u3::SEVEN - u3::FIVE, u3::TWO);
    assert_eq!(u3::SEVEN - u3::SIX, u3::ONE);
    assert_eq!(u3::SEVEN - u3::SEVEN, u3::ZERO);
}

#[test]
fn test_mult() {
    assert_eq!(u3::ZERO * u3::ZERO, u3::ZERO);
    assert_eq!(u3::ZERO * u3::ONE, u3::ZERO);
    assert_eq!(u3::ZERO * u3::TWO, u3::ZERO);
    assert_eq!(u3::ZERO * u3::THREE, u3::ZERO);
    assert_eq!(u3::ZERO * u3::FOUR, u3::ZERO);
    assert_eq!(u3::ZERO * u3::FIVE, u3::ZERO);
    assert_eq!(u3::ZERO * u3::SIX, u3::ZERO);
    assert_eq!(u3::ZERO * u3::SEVEN, u3::ZERO);
    assert_eq!(u3::ONE * u3::ZERO, u3::ZERO);
    assert_eq!(u3::ONE * u3::ONE, u3::ONE);
    assert_eq!(u3::ONE * u3::TWO, u3::TWO);
    assert_eq!(u3::ONE * u3::THREE, u3::THREE);
    assert_eq!(u3::ONE * u3::FOUR, u3::FOUR);
    assert_eq!(u3::ONE * u3::FIVE, u3::FIVE);
    assert_eq!(u3::ONE * u3::SIX, u3::SIX);
    assert_eq!(u3::ONE * u3::SEVEN, u3::SEVEN);
    assert_eq!(u3::TWO * u3::ZERO, u3::ZERO);
    assert_eq!(u3::TWO * u3::ONE, u3::TWO);
    assert_eq!(u3::TWO * u3::TWO, u3::FOUR);
    assert_eq!(u3::TWO * u3::THREE, u3::SIX);
    assert_eq!(u3::THREE * u3::ZERO, u3::ZERO);
    assert_eq!(u3::THREE * u3::ONE, u3::THREE);
    assert_eq!(u3::THREE * u3::TWO, u3::SIX);
    assert_eq!(u3::FOUR * u3::ZERO, u3::ZERO);
    assert_eq!(u3::FOUR * u3::ONE, u3::FOUR);
    assert_eq!(u3::FIVE * u3::ZERO, u3::ZERO);
    assert_eq!(u3::FIVE * u3::ONE, u3::FIVE);
    assert_eq!(u3::SIX * u3::ZERO, u3::ZERO);
    assert_eq!(u3::SIX * u3::ONE, u3::SIX);
    assert_eq!(u3::SEVEN * u3::ZERO, u3::ZERO);
    assert_eq!(u3::SEVEN * u3::ONE, u3::SEVEN);
}

#[test]
fn test_div() {
    assert_eq!(u3::ZERO / u3::ONE, u3::ZERO);
    assert_eq!(u3::ZERO / u3::TWO, u3::ZERO);
    assert_eq!(u3::ZERO / u3::THREE, u3::ZERO);
    assert_eq!(u3::ZERO / u3::FOUR, u3::ZERO);
    assert_eq!(u3::ZERO / u3::FIVE, u3::ZERO);
    assert_eq!(u3::ZERO / u3::SIX, u3::ZERO);
    assert_eq!(u3::ZERO / u3::SEVEN, u3::ZERO);
    assert_eq!(u3::ONE / u3::ONE, u3::ONE);
    assert_eq!(u3::TWO / u3::ONE, u3::TWO);
    assert_eq!(u3::TWO / u3::TWO, u3::ONE);
    assert_eq!(u3::THREE / u3::ONE, u3::THREE);
    assert_eq!(u3::THREE / u3::THREE, u3::ONE);
    assert_eq!(u3::FOUR / u3::ONE, u3::FOUR);
    assert_eq!(u3::FOUR / u3::TWO, u3::TWO);
    assert_eq!(u3::FOUR / u3::FOUR, u3::ONE);
    assert_eq!(u3::FIVE / u3::ONE, u3::FIVE);
    assert_eq!(u3::FIVE / u3::FIVE, u3::ONE);
    assert_eq!(u3::SIX / u3::ONE, u3::SIX);
    assert_eq!(u3::SIX / u3::TWO, u3::THREE);
    assert_eq!(u3::SIX / u3::THREE, u3::TWO);
    assert_eq!(u3::SIX / u3::SIX, u3::ONE);
    assert_eq!(u3::SEVEN / u3::ONE, u3::SEVEN);
    assert_eq!(u3::SEVEN / u3::SEVEN, u3::ONE);
}

#[test]
fn test_from_u3() {
    assert_eq!(u8::from(u3::ZERO), 0);
    assert_eq!(u8::from(u3::ONE), 1);
    assert_eq!(u8::from(u3::TWO), 2);
    assert_eq!(u8::from(u3::THREE), 3);
    assert_eq!(u8::from(u3::FOUR), 4);
    assert_eq!(u8::from(u3::FIVE), 5);
    assert_eq!(u8::from(u3::SIX), 6);
    assert_eq!(u8::from(u3::SEVEN), 7);
}

#[test]
fn test_from_u8() {
    assert_eq!(u3::from(0), u3::ZERO);
    assert_eq!(u3::from(1), u3::ONE);
    assert_eq!(u3::from(2), u3::TWO);
    assert_eq!(u3::from(3), u3::THREE);
    assert_eq!(u3::from(4), u3::FOUR);
    assert_eq!(u3::from(5), u3::FIVE);
    assert_eq!(u3::from(6), u3::SIX);
    assert_eq!(u3::from(7), u3::SEVEN);
}
