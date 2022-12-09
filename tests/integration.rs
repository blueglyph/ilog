// Copyright 2022 Redglyph
//
// Integration tests

use ilog::IntLog;

#[test]
fn log_u32() {
    let mut value1: u32 = 513;
    let value2: u32 = 512;
    value1 -= 2;
    let value1_log2 = value1.log2();
    let value1_log10 = value1.log10();
    let value2_log2 = u32::log2(value2);
    let value2_log10 = u32::log10(value2);
    assert_eq!(value1_log2, 8);
    assert_eq!(value1_log10, 2);
    assert_eq!(value2_log2, 9);
    assert_eq!(value2_log10, 2);

    let value1_chk_log2 = value1.checked_log2();
    let value1_chk_log10 = value1.checked_log10();
    let value2_chk_log2 = u32::checked_log2(value2);
    let value2_chk_log10 = u32::checked_log10(value2);
    let zero_chk_log2 = 0_u32.checked_log2();
    let zero_chk_log10 = u32::checked_log10(0);
    assert_eq!(value1_chk_log2, Some(8));
    assert_eq!(value1_chk_log10, Some(2));
    assert_eq!(value2_chk_log2, Some(9));
    assert_eq!(value2_chk_log10, Some(2));
    assert_eq!(zero_chk_log2, None);
    assert_eq!(zero_chk_log10, None);
}

#[test]
fn log_u64() {
    let mut value1: u64 = 513;
    let value2: u64 = 512;
    value1 -= 2;
    let value1_log2 = value1.log2();
    let value1_log10 = value1.log10();
    let value2_log2 = u64::log2(value2);
    let value2_log10 = u64::log10(value2);
    assert_eq!(value1_log2, 8);
    assert_eq!(value1_log10, 2);
    assert_eq!(value2_log2, 9);
    assert_eq!(value2_log10, 2);

    let value1_chk_log2 = value1.checked_log2();
    let value1_chk_log10 = value1.checked_log10();
    let value2_chk_log2 = u64::checked_log2(value2);
    let value2_chk_log10 = u64::checked_log10(value2);
    let zero_chk_log2 = 0_u64.checked_log2();
    let zero_chk_log10 = u64::checked_log10(0);
    assert_eq!(value1_chk_log2, Some(8));
    assert_eq!(value1_chk_log10, Some(2));
    assert_eq!(value2_chk_log2, Some(9));
    assert_eq!(value2_chk_log10, Some(2));
    assert_eq!(zero_chk_log2, None);
    assert_eq!(zero_chk_log10, None);
}

#[test]
fn log_u128() {
    let mut value1: u128 = 513;
    let value2: u128 = 512;
    value1 -= 2;
    let value1_log2 = value1.log2();
    let value1_log10 = value1.log10();
    let value2_log2 = u128::log2(value2);
    let value2_log10 = u128::log10(value2);
    assert_eq!(value1_log2, 8);
    assert_eq!(value1_log10, 2);
    assert_eq!(value2_log2, 9);
    assert_eq!(value2_log10, 2);

    let value1_chk_log2 = value1.checked_log2();
    let value1_chk_log10 = value1.checked_log10();
    let value2_chk_log2 = u128::checked_log2(value2);
    let value2_chk_log10 = u128::checked_log10(value2);
    let zero_chk_log2 = 0_u128.checked_log2();
    let zero_chk_log10 = u128::checked_log10(0);
    assert_eq!(value1_chk_log2, Some(8));
    assert_eq!(value1_chk_log10, Some(2));
    assert_eq!(value2_chk_log2, Some(9));
    assert_eq!(value2_chk_log10, Some(2));
    assert_eq!(zero_chk_log2, None);
    assert_eq!(zero_chk_log10, None);
}
