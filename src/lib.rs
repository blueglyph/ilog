use num_traits::Zero;

mod tests;

// =============================================================================================

/// Provides logarithms for integer types.
pub trait IntLog {
    /// Returns the largest integer less than or equal to the base 10 logarithm of the integer.
    fn log10(self) -> usize;

    /// Returns the largest integer less than or equal to the base 2 logarithm of the integer.
    fn log2(self) -> usize;

    /// Checked base 10 logarithm. Returns the largest integer less than or equal to the base 10
    /// logarithm of the integer, or None if it doesn't exist.
    ///
    /// # Examples
    /// ```
    /// # use ilog::IntLog;
    /// assert_eq!(100_u32.checked_log10(), Some(2));
    /// assert_eq!(u32::checked_log10(99), Some(1));
    /// assert_eq!(0_u32.checked_log10(), None);
    /// ```
    #[inline]
    fn checked_log10(self) -> Option<usize> where Self: PartialOrd + Sized + Zero {
        if self > Self::zero() { Some(self.log10()) } else { None }
    }

    /// Checked base 10 logarithm. Returns the largest integer less than or equal to the base 10
    /// logarithm of the integer, or None if it doesn't exist.
    ///
    /// # Examples
    /// ```
    /// # use ilog::IntLog;
    /// assert_eq!(64_u32.checked_log2(), Some(6));
    /// assert_eq!(u32::checked_log2(63), Some(5));
    /// assert_eq!(0_u32.checked_log2(), None);
    /// ```
    #[inline]
    fn checked_log2(self) -> Option<usize> where Self: PartialOrd + Sized + Zero {
        if self > Self::zero() { Some(self.log2()) } else { None }
    }
}

// ---------------------------------------------------------------------------------------------

const LOG10_U32_TABLE: [u32; 11] = [0, 9, 99, 999, 9999, 99999, 999999, 9999999, 99999999, 999999999,
    0xFFFFFFFF];

impl IntLog for u32 {
    /// Returns the largest integer less than or equal to the base 10 logarithm of the integer.
    ///
    /// # Examples
    /// ```
    /// # use ilog::IntLog;
    /// let hundred: u32 = 100;
    /// assert_eq!(hundred.log10(), 2);
    /// assert_eq!(u32::log10(99), 1);
    /// ```
    #[inline]
    fn log10(self) -> usize {
        let y = (19 * (31 - self.leading_zeros() as usize)) >> 6;
        y + ((LOG10_U32_TABLE[y + 1].wrapping_sub(self)) >> 31) as usize
    }

    /// Returns the largest integer less than or equal to the base 2 logarithm of the integer.
    ///
    /// # Examples
    /// ```
    /// # use ilog::IntLog;
    /// let value: u32 = 128;
    /// assert_eq!(value.log2(), 7);
    /// assert_eq!(u32::log2(127), 6);
    /// ```
    #[inline]
    fn log2(self) -> usize {
        31 - self.leading_zeros() as usize
    }
}

// ---------------------------------------------------------------------------------------------

const LOG10_U64_TABLE: [u64; 21] = [0, 9, 99, 999, 9999, 99999, 999999, 9999999, 99999999, 999999999,
    9999999999, 99999999999, 999999999999, 9999999999999, 99999999999999, 999999999999999,
    9999999999999999, 99999999999999999, 999999999999999999, 9999999999999999999,
    0xFFFFFFFF_FFFFFFFF];

impl IntLog for u64 {

    /// Returns the largest integer less than or equal to the base 10 logarithm of the integer.
    ///
    /// # Examples
    /// ```
    /// # use ilog::IntLog;
    /// let thousand: u64 = 1000;
    /// assert_eq!(thousand.log10(), 3);
    /// assert_eq!(u64::log10(999), 2);
    /// ```
    #[inline]
    fn log10(self) -> usize {
        let y = (19 * (63 - self.leading_zeros() as usize)) >> 6;
        y + ((LOG10_U64_TABLE[y + 1].wrapping_sub(self)) >> 63) as usize
    }

    /// Returns the largest integer less than or equal to the base 2 logarithm of the integer.
    ///
    /// # Examples
    /// ```
    /// # use ilog::IntLog;
    /// let value: u64 = 256;
    /// assert_eq!(value.log2(), 8);
    /// assert_eq!(u64::log2(255), 7);
    /// ```
    #[inline]
    fn log2(self) -> usize {
        63 - self.leading_zeros() as usize
    }
}

// ---------------------------------------------------------------------------------------------
