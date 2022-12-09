// Copyright 2022 Redglyph
//
// Unit tests

#![cfg(test)]

// ---------------------------------------------------------------------------------------------

/// Unit tests of logarithms for unsigned types
macro_rules! test_unsigned_log {
    (
        $Name10: ident,         // test name for log10
        $Name2: ident,          // test name for log2
        $SelfT: ty,             // type to test
        $UpperLoop10: expr,     // top value of 10-power loop
        $Nine: expr,            // extra 99...99 value to test
        $NineLog10: expr,       // expected log10 result of extra 99...99
        $MaxLog10: expr,        // expected log10(type::MAX)
        $Msb: expr              // MSB index of type
    ) => {
        #[test]
        fn $Name10() {
            // tests powers of 10
            let mut value: $SelfT = 1;
            for i in 1..=$UpperLoop10 {
                value *= 10;
                assert_eq!((value - 1).log10(), i - 1, "loop.1 at i = {i}");
                assert_eq!(value.log10(), i, "loop.2 at i = {i}");
            }
            // tests powers of 2
            value = 1;
            let mut threshold = 10;
            let mut exp = 0;
            let mut error = false;
            for _ in 1..=$Msb {
                let new = value*2;
                if new >= threshold {
                    exp += 1;
                    threshold = threshold.checked_mul(10).unwrap_or(<$SelfT>::MAX);
                }
                value = new;
                let result = <$SelfT>::log10(value);
                if result != exp {
                    error = true;
                    println!("mismatch for v = {value}: {result} instead of {exp}");
                }
            }
            // tests key & forbidden values
            assert!(!error, "0");
            assert_eq!(<$SelfT>::log10(1), 0, "1");
            assert_eq!($Nine.log10(), $NineLog10, "2");
            assert_eq!(<$SelfT>::MAX.log10(), $MaxLog10, "3");
            assert_eq!(<$SelfT>::checked_log10(0), None, "4");
            assert_eq!(<$SelfT>::checked_log10(1), Some(0), "5");
            assert_eq!(<$SelfT>::checked_log10(<$SelfT>::MAX), Some($MaxLog10), "6");
        }

        #[test]
        fn $Name2() {
            // tests powers of 2
            let mut value: $SelfT = 1;
            for i in 1..=$Msb {
                value *= 2;
                assert_eq!((value - 1).log2(), i - 1, "loop.1 at i = {i}");
                assert_eq!(value.log2(), i, "loop.2 at i = {i}");
            }
            // tests key & forbidden values
            assert_eq!(<$SelfT>::MAX.log2(), $Msb, "1");
            assert_eq!(<$SelfT>::checked_log2(0), None, "2");
            assert_eq!(<$SelfT>::checked_log2(1), Some(0), "3");
            assert_eq!(<$SelfT>::checked_log2(<$SelfT>::MAX), Some($Msb), "4");
        }
    }
}

/// Unit tests of logarithms for signed types
macro_rules! test_signed_log {
    (
        $Name10: ident,         // test name for log10
        $Name2: ident,          // test name for log2
        $SelfT: ty,             // type to test
        $UpperLoop10: expr,     // top value of 10-power loop
        $Nine: expr,            // extra 99...99 value to test
        $NineLog10: expr,       // expected log10 result of extra 99...99
        $MaxLog10: expr,        // expected log10(type::MAX)
        $Msb: expr              // MSB index of type
    ) => {
        #[test]
        fn $Name10() {
            // tests powers of 10
            let mut value: $SelfT = 1;
            for i in 1..=$UpperLoop10 {
                value *= 10;
                assert_eq!((value - 1).log10(), i - 1, "loop.1 at i = {i}");
                assert_eq!(value.log10(), i, "loop.2 at i = {i}");
            }
            // tests powers of 2
            value = 1;
            let mut threshold = 10;
            let mut exp = 0;
            let mut error = false;
            for _ in 1..=$Msb {
                let new = value*2;
                if new >= threshold {
                    exp += 1;
                    threshold = threshold.checked_mul(10).unwrap_or(<$SelfT>::MAX);
                }
                value = new;
                let result = <$SelfT>::log10(value);
                if result != exp {
                    error = true;
                    println!("mismatch for v = {value}: {result} instead of {exp}");
                }
            }
            // tests key & forbidden values
            assert!(!error, "0");
            assert_eq!(<$SelfT>::log10(1), 0, "1");
            assert_eq!($Nine.log10(), $NineLog10, "2");
            assert_eq!(<$SelfT>::MAX.log10(), $MaxLog10, "3");
            assert_eq!(<$SelfT>::checked_log10(0), None, "4");
            assert_eq!(<$SelfT>::checked_log10(-1), None, "5");
            assert_eq!(<$SelfT>::checked_log10(1), Some(0), "6");
            assert_eq!(<$SelfT>::checked_log10(<$SelfT>::MAX), Some($MaxLog10), "7");
        }

        #[test]
        fn $Name2() {
            // tests powers of 2
            let mut value: $SelfT = 1;
            for i in 1..=$Msb {
                value *= 2;
                assert_eq!((value - 1).log2(), i - 1, "loop.1 at i = {i}");
                assert_eq!(value.log2(), i, "loop.2 at i = {i}");
            }
            // tests key & forbidden values
            assert_eq!(<$SelfT>::MAX.log2(), $Msb, "1");
            assert_eq!(<$SelfT>::checked_log2(0), None, "2");
            assert_eq!(<$SelfT>::checked_log2(-1), None, "3");
            assert_eq!(<$SelfT>::checked_log2(1), Some(0), "4");
            assert_eq!(<$SelfT>::checked_log2(<$SelfT>::MAX), Some($Msb), "5");
        }
    }
}

// ---------------------------------------------------------------------------------------------

use crate::IntLog;
test_unsigned_log! { log10_u8, log2_u8, u8, 2, 99_u8, 1, 2, 7 }
test_signed_log!   { log10_i8, log2_i8, i8, 2, 99_u8, 1, 2, 6 }

test_unsigned_log! { log10_u16, log2_u16, u16, 4, 9_999_u16, 3, 4, 15 }
test_signed_log!   { log10_i16, log2_i16, i16, 4, 9_999_u16, 3, 4, 14 }
#[cfg(target_pointer_width = "16")]
test_unsigned_log! { log10_usize, log2_usize, usize, 4, 9_999_u16, 3, 4, 15 }
#[cfg(target_pointer_width = "16")]
test_signed_log!   { log10_usize, log2_usize, usize, 4, 9_999_u16, 3, 4, 14 }

test_unsigned_log! { log10_u32, log2_u32, u32, 9, 999_999_999_u32, 8, 9, 31 }
test_signed_log!   { log10_i32, log2_i32, i32, 9, 999_999_999_u32, 8, 9, 30 }
#[cfg(target_pointer_width = "32")]
test_unsigned_log! { log10_usize, log2_usize, usize, 9, 999_999_999_u32, 8, 9, 31 }
#[cfg(target_pointer_width = "32")]
test_signed_log!   { log10_usize, log2_usize, usize, 9, 999_999_999_u32, 8, 9, 30 }

test_unsigned_log! { log10_u64, log2_u64, u64, 19, 9999999999999999999_u64, 18, 19, 63 }
test_signed_log!   { log10_i64, log2_i64, i64, 18, 9999999999999999999_u64, 18, 18, 62 }
#[cfg(target_pointer_width = "64")]
test_unsigned_log! { log10_usize, log2_usize, usize, 19, 9999999999999999999_u64, 18, 19, 63 }
#[cfg(target_pointer_width = "64")]
test_signed_log!   { log10_isize, log2_isize, isize, 18, 9999999999999999999_u64, 18, 18, 62 }

test_unsigned_log! { log10_u128, log2_u128,  u128, 37, 99999999999999999999999999999999999999_u128, 37, 38, 127 }
test_signed_log!   { log10_i128, log2_i128,  i128, 37, 99999999999999999999999999999999999999_u128, 37, 38, 126 }

// ---------------------------------------------------------------------------------------------
