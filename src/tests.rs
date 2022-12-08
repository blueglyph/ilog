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
/// Computes floor(log(base=2, x))
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
/// Computes floor(log(base=10, x))
fn log10_u64() {
    let mut value = 1_u64;
    for i in 1..=18 {
        value *= 10;
        assert_eq!((value - 1).log10(), i - 1);
        assert_eq!(value.log10(), i);
    }
    assert_eq!(1_u64.log10(), 0);
    assert_eq!(9_999_999_999_999_999_999_u64.log10(), 18);
    assert_eq!(u64::MAX.log10(), 19);
    assert_eq!(u64::checked_log10(0), None);
    assert_eq!(u64::checked_log10(1), Some(0));
    assert_eq!(u64::checked_log10(u64::MAX), Some(19));
}

#[test]
/// Computes floor(log(base=2, x))
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

