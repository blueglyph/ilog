// Copyright 2022 Redglyph
//
// Unit tests

#![cfg(test)]
use crate::IntLog;

#[test]
fn log10_u32() {
    let mut value = 1_u32;
    for i in 1..=9 {
        value *= 10;
        assert_eq!((value - 1).log10(), i - 1);
        assert_eq!(value.log10(), i);
    }
    assert_eq!(1_u32.log10(), 0);
    assert_eq!(999_999_999_u32.log10(), 8);
    assert_eq!(u32::MAX.log10(), 9);
    assert_eq!(u32::checked_log10(0), None);
    assert_eq!(u32::checked_log10(1), Some(0));
    assert_eq!(u32::checked_log10(u32::MAX), Some(9));
}

#[test]
fn log2_u32() {
    let mut value = 1_u32;
    for i in 1..=31 {
        value *= 2;
        assert_eq!((value - 1).log2(), i - 1);
        assert_eq!(value.log2(), i);
    }
    assert_eq!(u32::MAX.log2(), 31);
    assert_eq!(u32::checked_log2(0), None);
    assert_eq!(u32::checked_log2(1), Some(0));
    assert_eq!(u32::checked_log2(u32::MAX), Some(31));
}

#[test]
fn log10_u64() {
    let mut value = 1_u64;
    for i in 1..=19 {
        value *= 10;
        assert_eq!((value - 1).log10(), i - 1);
        assert_eq!(value.log10(), i);
    }
    assert_eq!(1_u64.log10(), 0);
    assert_eq!(u64::MAX.log10(), 19);
    assert_eq!(u64::checked_log10(0), None);
    assert_eq!(u64::checked_log10(1), Some(0));
    assert_eq!(u64::checked_log10(u64::MAX), Some(19));
}

#[test]
fn log2_u64() {
    let mut value = 1_u64;
    for i in 1..=63 {
        value *= 2;
        assert_eq!((value - 1).log2(), i - 1);
        assert_eq!(value.log2(), i);
    }
    assert_eq!(u64::MAX.log2(), 63);
    assert_eq!(u64::checked_log2(0), None);
    assert_eq!(u64::checked_log2(1), Some(0));
    assert_eq!(u64::checked_log2(u64::MAX), Some(63));
}

#[test]
fn log10_u128() {
    let mut value = 1_u128;
    for i in 1..=37 {
        value *= 10;
        assert_eq!((value - 1).log10(), i - 1);
        assert_eq!(value.log10(), i);
    }
    assert_eq!(1_u128.log10(), 0);
    assert_eq!(99999999999999999999999999999999999999_u128.log10(), 37);
    assert_eq!(u128::MAX.log10(), 38);
    assert_eq!(u128::checked_log10(0), None);
    assert_eq!(u128::checked_log10(1), Some(0));
    assert_eq!(u128::checked_log10(u128::MAX), Some(38));
}

#[test]
fn log2_u128() {
    let mut value = 1_u128;
    for i in 1..=127 {
        value *= 2;
        assert_eq!((value - 1).log2(), i - 1);
        assert_eq!(value.log2(), i);
    }
    assert_eq!(u128::MAX.log2(), 127);
    assert_eq!(u128::checked_log2(0), None);
    assert_eq!(u128::checked_log2(1), Some(0));
    assert_eq!(u128::checked_log2(u128::MAX), Some(127));
}

