// Copyright 2022 Redglyph
//
// Base 10 and 2 logarithm functions for integer types:
// u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize

use num_traits::Zero;

mod tests;

// =============================================================================================

/// Trait that provides logarithms for integer types.
///
/// The [IntLog::log2] and [IntLog::log10] methods are optimized for the integer width and are
/// `[inline]` as long as the code remains small enough. They typically use constant tables
/// that are only stored once, even if the methods using them are inlined multiple times.
///
/// The **checked** versions of the methods, [IntLog::checked_log2] and [IntLog::checked_log10],
/// return `None` if the logarithm is undefined for the parameter value, whereas the unchecked
/// methods mentioned above simply panic. A default implementation is provided in the trait, and in
/// most cases they needn't be overidden.
pub trait IntLog {
    /// Returns the largest integer less than or equal to the base 10 logarithm of the integer.
    ///
    /// Logarithms are only defined on positive values, calling `log10` with a null or a negative
    /// argument may trigger a panic or return an undefined value.
    /// See [checked_log10](Self::checked_log10) for a method that checks its argument first.
    ///
    /// # Examples
    /// ```
    /// # use ilog::IntLog;
    /// let value: u64 = 100;
    /// assert_eq!(value.log10(), 2);
    /// assert_eq!(i32::log10(99), 1);
    /// ```
    fn log10(self) -> usize;

    /// Returns the largest integer less than or equal to the base 2 logarithm of the integer.
    ///
    /// Logarithms are only defined on positive values, calling `log10` with a null or a negative
    /// argument may trigger a panic or return an undefined value.
    /// See [checked_log2](Self::checked_log2) for a method that checks its argument first.
    ///
    /// # Examples
    /// ```
    /// # use ilog::IntLog;
    /// let value: u64 = 64;
    /// assert_eq!(value.log2(), 6);
    /// assert_eq!(i32::log2(63), 5);
    /// ```
    fn log2(self) -> usize;

    /// Checked base 10 logarithm. Returns the largest integer less than or equal to the base 10
    /// logarithm of the integer, or `None` if it doesn't exist.
    ///
    /// # Examples
    /// ```
    /// # use ilog::IntLog;
    /// assert_eq!(100_u32.checked_log10(), Some(2));
    /// assert_eq!(u64::checked_log10(99), Some(1));
    /// assert_eq!(0_u32.checked_log10(), None);
    /// ```
    #[inline]
    fn checked_log10(self) -> Option<usize> where Self: PartialOrd + Sized + Zero {
        if self > Self::zero() { Some(self.log10()) } else { None }
    }

    /// Checked base 10 logarithm. Returns the largest integer less than or equal to the base 10
    /// logarithm of the integer, or `None` if it doesn't exist.
    ///
    /// # Examples
    /// ```
    /// # use ilog::IntLog;
    /// assert_eq!(64_u32.checked_log2(), Some(6));
    /// assert_eq!(u64::checked_log2(63), Some(5));
    /// assert_eq!(0_u32.checked_log2(), None);
    /// ```
    #[inline]
    fn checked_log2(self) -> Option<usize> where Self: PartialOrd + Sized + Zero {
        if self > Self::zero() { Some(self.log2()) } else { None }
    }
}

// ---------------------------------------------------------------------------------------------

macro_rules! impl_unsigned_log {
    ($SelfT: ty, $Msb: expr, $ApproxMul: expr, $ApproxShr: expr, $Table: ident) => {
        impl IntLog for $SelfT {
            #[inline]
            fn log10(self) -> usize {
                let y = ($ApproxMul * ($Msb - self.leading_zeros() as usize)) >> $ApproxShr;
                y + (($Table[y + 1] as $SelfT).wrapping_sub(self) >> $Msb) as usize
            }

            #[inline]
            fn log2(self) -> usize {
                $Msb - self.leading_zeros() as usize
            }
        }
    }
}

macro_rules! impl_signed_log {
    ($SelfT: ty, $UnsignedT: ty) => {
        impl IntLog for $SelfT {
            #[inline]
            fn log10(self) -> usize {
                <$UnsignedT>::log10(self as $UnsignedT)
            }

            #[inline]
            fn log2(self) -> usize {
                <$UnsignedT>::log2(self as $UnsignedT)
            }
        }
    }
}

// ---------------------------------------------------------------------------------------------
const LOG10_U8_TABLE: [u8; 4] = [0, 9, 99, u8::MAX];

impl_unsigned_log! { u8, 7, 19, 6, LOG10_U8_TABLE }
impl_signed_log! { i8, u8 }

const LOG10_U16_TABLE: [u16; 6] = [0, 9, 99, 999, 9999, u16::MAX];

impl_unsigned_log! { u16, 15, 18, 6, LOG10_U16_TABLE }
impl_signed_log! { i16, u16 }
#[cfg(target_pointer_width = "16")]
impl_unsigned_log! { usize, 15, 18, 6, LOG10_U16_TABLE }
#[cfg(target_pointer_width = "16")]
impl_signed_log! { isize, u16 }

const LOG10_U32_TABLE: [u32; 11] = [0, 9, 99, 999, 9999, 99999, 999999, 9999999, 99999999, 999999999, u32::MAX];

impl_unsigned_log! { u32, 31, 19, 6, LOG10_U32_TABLE }
impl_signed_log! { i32, u32 }
#[cfg(target_pointer_width = "32")]
impl_unsigned_log! { usize, 31, 19, 6, LOG10_U32_TABLE }
#[cfg(target_pointer_width = "32")]
impl_signed_log! { isize, u32 }

const LOG10_U64_TABLE: [u64; 20] = [0, 9, 99, 999, 9999, 99999, 999999, 9999999, 99999999, 999999999,
    9999999999, 99999999999, 999999999999, 9999999999999, 99999999999999, 999999999999999,
    9999999999999999, 99999999999999999, 999999999999999999, 9999999999999999999];

impl_unsigned_log! { u64, 63, 19, 6, LOG10_U64_TABLE }
impl_signed_log! { i64, u64 }
#[cfg(target_pointer_width = "64")]
impl_unsigned_log! { usize, 63, 19, 6, LOG10_U64_TABLE }
#[cfg(target_pointer_width = "64")]
impl_signed_log! { isize, u64 }

const LOG10_U128_TABLE: [u128; 40] = [0, 9, 99, 999, 9999, 99999, 999999, 9999999, 99999999, 999999999,
    9999999999, 99999999999, 999999999999, 9999999999999, 99999999999999, 999999999999999,
    9999999999999999, 99999999999999999, 999999999999999999, 9999999999999999999, 99999999999999999999,
    999999999999999999999, 9999999999999999999999, 99999999999999999999999, 999999999999999999999999,
    9999999999999999999999999, 99999999999999999999999999, 999999999999999999999999999, 9999999999999999999999999999,
    99999999999999999999999999999, 999999999999999999999999999999, 9999999999999999999999999999999,
    99999999999999999999999999999999, 999999999999999999999999999999999, 9999999999999999999999999999999999,
    99999999999999999999999999999999999, 999999999999999999999999999999999999, 9999999999999999999999999999999999999,
    99999999999999999999999999999999999999, u128::MAX];

impl_unsigned_log! { u128, 127, 77, 8, LOG10_U128_TABLE }
impl_signed_log! { i128, u128 }

// ---------------------------------------------------------------------------------------------
