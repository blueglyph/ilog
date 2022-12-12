// Copyright 2022 Redglyph
//
// Integration tests: tests that all the functionalities are accessible and work as expected.

#![cfg(test)]

use ilog::IntLog;

/// Integration tests of logarithms for signed and unsigned types.
macro_rules! intg_log {
    (
        $Name: ident,       // test name
        $SelfT: ty,         // type to test
        $Exp10: expr,       // expected log10 value
        $Exp2: expr,        // expected log2 value (used to calculate the test value)
        $Forbidden: expr    // extra forbidden value (e.g. negative value for signed types)
    ) => {
        #[test]
        fn $Name() {
            let mut value1: $SelfT = 1 << $Exp2;
            let value2: $SelfT = 1 << $Exp2;
            value1 -= 1;
            // form 1
            let value1_log2 = value1.log2();
            let value1_log10 = value1.log10();
            // form 2
            let value2_log2 = <$SelfT>::log2(value2);
            let value2_log10 = <$SelfT>::log10(value2);
            // reference
            let ref_value1 = &value1;
            let ref_value1_log2 = ref_value1.log2();
            let ref_value1_log10 = ref_value1.log10();

            assert_eq!(value1_log2, $Exp2 - 1, "log2(value1)");
            assert_eq!(value1_log10, $Exp10, "log10(value1)");
            assert_eq!(value2_log2, $Exp2, "log2(value2)");
            assert_eq!(value2_log10, $Exp10, "log10(value2)");
            assert_eq!(ref_value1_log2, $Exp2 - 1, "log2(ref_value1)");
            assert_eq!(ref_value1_log10, $Exp10, "log10(ref_value1)");

            // form 1
            let value1_chk_log2 = value1.checked_log2();
            let value1_chk_log10 = value1.checked_log10();
            // form 2
            let value2_chk_log2 = <$SelfT>::checked_log2(value2);
            let value2_chk_log10 = <$SelfT>::checked_log10(value2);
            // reference
            let ref_value1_chk_log2 = ref_value1.checked_log2();
            let ref_value1_chk_log10 = ref_value1.checked_log10();
            // forbidden values
            let zero_chk_log2 = <$SelfT>::checked_log2(0);
            let zero_chk_log10 = <$SelfT>::checked_log10(0);
            let forbid_chk_log2 = <$SelfT>::checked_log2($Forbidden);
            let forbid_chk_log10 = <$SelfT>::checked_log10($Forbidden);

            assert_eq!(value1_chk_log2, Some($Exp2 - 1), "checked_log2(value1)");
            assert_eq!(value1_chk_log10, Some($Exp10), "checked_log10(value1)");
            assert_eq!(value2_chk_log2, Some($Exp2), "checked_log2(value2)");
            assert_eq!(value2_chk_log10, Some($Exp10), "checked_log10(value2)");
            assert_eq!(ref_value1_chk_log2, Some($Exp2 - 1), "checked_log2(ref_value1)");
            assert_eq!(ref_value1_chk_log10, Some($Exp10), "checked_log10(ref_value1)");
            assert_eq!(zero_chk_log2, None, "checked_log2(0)");
            assert_eq!(zero_chk_log10, None, "checked_log10(0)");
            assert_eq!(forbid_chk_log2, None, "checked_log2({})", $Forbidden);
            assert_eq!(forbid_chk_log10, None, "checked_log10({})", $Forbidden);
        }
    }
}

intg_log!(log_u8_intg, u8, 1, 6, 0);
intg_log!(log_i8_intg, i8, 1, 6, -1);

intg_log!(log_u16_intg, u16, 2, 9, 0);
intg_log!(log_i16_intg, i16, 2, 9, -1);
#[cfg(target_pointer_width = "16")]
intg_log!(log_usize_intg, usize, 2, 9, 0);
#[cfg(target_pointer_width = "16")]
intg_log!(log_isize_intg, isize, 2, 9, -1);

intg_log!(log_u32_intg, u32, 2, 9, 0);
intg_log!(log_i32_intg, i32, 2, 9, -1);
#[cfg(target_pointer_width = "32")]
intg_log!(log_usize_intg, usize, 2, 9, 0);
#[cfg(target_pointer_width = "32")]
intg_log!(log_isize_intg, isize, 2, 9, -1);

intg_log!(log_u64_intg, u64, 2, 9, 0);
intg_log!(log_i64_intg, i64, 2, 9, -1);
#[cfg(target_pointer_width = "64")]
intg_log!(log_usize_intg, usize, 2, 9, 0);
#[cfg(target_pointer_width = "64")]
intg_log!(log_isize_intg, isize, 2, 9, -1);

intg_log!(log_u128_intg, u128, 2, 9, 0);
intg_log!(log_i128_intg, i128, 2, 9, -1);

#[test]
fn log_u32_intgx() {
    let mut value1: u32 = 1 << 9;
    let value2: u32 = 1 << 9;
    value1 -= 1;

    // direct log methods
    let value1_log2 = value1.log2();
    let value1_log10 = value1.log10();

    let value2_log2 = <u32>::log2(value2);
    let value2_log10 = <u32>::log10(value2);

    assert_eq!(value1_log2, 9 - 1, "value1.log2()");
    assert_eq!(value1_log10, 2, "value1.log10()");
    assert_eq!(value2_log2, 9, "log2(value2)");
    assert_eq!(value2_log10, 2, "log10(value2)");

    // checked log methods
    let value1_chk_log2 = value1.checked_log2();
    let value1_chk_log10 = value1.checked_log10();

    let value2_chk_log2 = <u32>::checked_log2(value2);
    let value2_chk_log10 = <u32>::checked_log10(value2);

    let zero_chk_log2 = <u32>::checked_log2(0);
    let zero_chk_log10 = <u32>::checked_log10(0);
    let forbid_chk_log2 = <u32>::checked_log2(0);
    let forbid_chk_log10 = <u32>::checked_log10(0);

    assert_eq!(value1_chk_log2, Some(9 - 1), "value1.checked_log2()");
    assert_eq!(value1_chk_log10, Some(2), "value1.checked_log10()");
    assert_eq!(value2_chk_log2, Some(9), "checked_log2(value2)");
    assert_eq!(value2_chk_log10, Some(2), "checked_log10(value2)");
    assert_eq!(zero_chk_log2, None, "checked_log2(0)");
    assert_eq!(zero_chk_log10, None, "checked_log10(0)");
    assert_eq!(forbid_chk_log2, None, "checked_log2({})", 0);
    assert_eq!(forbid_chk_log10, None, "checked_log10({})", 0);

    // references
    let ref_value1 = &value1;
    let ref_value1_log2 = ref_value1.log2();
    let ref_value1_log10 = ref_value1.log10();
    let ref_value1_chk_log2 = ref_value1.checked_log2();
    let ref_value1_chk_log10 = ref_value1.checked_log10();

    assert_eq!(ref_value1_log2, 9 - 1, "ref_value1.log2()");
    assert_eq!(ref_value1_log10, 2, "ref_value1.log10()");
    assert_eq!(ref_value1_chk_log2, Some(9 - 1), "ref_value1.checked_log2()");
    assert_eq!(ref_value1_chk_log10, Some(2), "ref_value1.checked_log10()");

    // mutable references
    let refmut_value1 = &mut value1;
    let refmut_value1_log2 = refmut_value1.log2();
    let refmut_value1_log10 = refmut_value1.log10();
    let refmut_value1_chk_log2 = refmut_value1.checked_log2();
    let refmut_value1_chk_log10 = refmut_value1.checked_log10();

    assert_eq!(refmut_value1_log2, 9 - 1, "refmut_value1.log2()");
    assert_eq!(refmut_value1_log10, 2, "refmut_value1.log10()");
    assert_eq!(refmut_value1_chk_log2, Some(9 - 1), "refmut_value1.checked_log2()");
    assert_eq!(refmut_value1_chk_log10, Some(2), "refmut_value1.checked_log10()");
}
