//! Newtype wrappers for timestamps in different time scales.
//!
//! Each type carries an explicit scale label so the compiler prevents mixing
//! UTC, TAI, and GPST values at call sites.
//!
//! # Seconds types
//!
//! | Type | Scale | Inner |
//! |------|-------|-------|
//! | [`UtcUnixSeconds`] | UTC | `i64` |
//! | [`TaiSeconds`] | TAI | `i64` |
//! | [`GpstSeconds`] | GPST | `i64` |
//!
//! # Nanosecond types
//!
//! | Type | Scale | Inner |
//! |------|-------|-------|
//! | [`UtcUnixNanos`] | UTC | `i128` |
//! | [`TaiNanos`] | TAI | `i128` |
//! | [`GpstNanos`] | GPST | `i128` |
//!
//! Use [`From`] to promote seconds to nanoseconds (lossless), and
//! [`to_seconds_floor()`](UtcUnixNanos::to_seconds_floor) to truncate back.

use core::fmt;

/// Unix-like seconds count in the UTC scale.
///
/// This is the standard POSIX timestamp — seconds since 1970-01-01T00:00:00 UTC,
/// with leap seconds folded (the 61st second `23:59:60` shares the same POSIX
/// value as the following `00:00:00`).
///
/// Use [`LeapSeconds::utc_to_tai`](crate::LeapSeconds::utc_to_tai) to convert
/// to TAI, or promote to [`UtcUnixNanos`] via `.into()`.
///
/// # Example
///
/// ```
/// use leap_sec::UtcUnixSeconds;
///
/// let utc = UtcUnixSeconds(1_700_000_000); // 2023-11-14 22:13:20 UTC
/// assert_eq!(format!("{utc}"), "1700000000 UTC");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UtcUnixSeconds(pub i64);

/// Continuous seconds count in the TAI scale (no leap seconds).
///
/// TAI is ahead of UTC by a varying number of whole seconds (37 as of 2017-01-01).
/// The inner `i64` counts seconds since the Unix epoch in TAI.
///
/// Use [`LeapSeconds::tai_to_utc`](crate::LeapSeconds::tai_to_utc) to convert
/// back to UTC, or [`tai_to_gpst`](crate::tai_to_gpst) to convert to GPST.
///
/// # Example
///
/// ```
/// use leap_sec::TaiSeconds;
///
/// let tai = TaiSeconds(1_700_000_037);
/// assert_eq!(format!("{tai}"), "1700000037 TAI");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TaiSeconds(pub i64);

/// Continuous seconds count in the GPS time scale.
///
/// GPST is offset from TAI by exactly 19 seconds: `GPST = TAI − 19`.
/// The inner `i64` counts seconds since the Unix epoch in GPST.
///
/// Convert to/from TAI with [`tai_to_gpst`](crate::tai_to_gpst) and
/// [`gpst_to_tai`](crate::gpst_to_tai). Convert to/from UTC via
/// [`LeapSeconds::utc_to_gpst`](crate::LeapSeconds::utc_to_gpst).
///
/// # Example
///
/// ```
/// use leap_sec::GpstSeconds;
///
/// let gpst = GpstSeconds(1_700_000_018);
/// assert_eq!(format!("{gpst}"), "1700000018 GPST");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GpstSeconds(pub i64);

/// Unix-epoch nanoseconds in the UTC scale.
///
/// Uses `i128` to hold the full range of Unix nanoseconds without overflow.
/// Promote from [`UtcUnixSeconds`] via `.into()`, truncate back with
/// [`to_seconds_floor`](Self::to_seconds_floor).
///
/// # Example
///
/// ```
/// use leap_sec::{UtcUnixSeconds, UtcUnixNanos};
///
/// let ns = UtcUnixNanos(1_700_000_000_500_000_000);
/// assert_eq!(format!("{ns}"), "1700000000500000000 UTC");
///
/// // Promote from seconds
/// let promoted: UtcUnixNanos = UtcUnixSeconds(1_700_000_000).into();
/// assert_eq!(promoted, UtcUnixNanos(1_700_000_000_000_000_000));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UtcUnixNanos(pub i128);

/// Continuous nanoseconds in the TAI scale.
///
/// Uses `i128` to hold the full range of nanoseconds without overflow.
/// Promote from [`TaiSeconds`] via `.into()`, truncate back with
/// [`to_seconds_floor`](Self::to_seconds_floor).
///
/// # Example
///
/// ```
/// use leap_sec::{TaiSeconds, TaiNanos};
///
/// let ns = TaiNanos(1_700_000_037_500_000_000);
/// assert_eq!(format!("{ns}"), "1700000037500000000 TAI");
///
/// let promoted: TaiNanos = TaiSeconds(1_700_000_037).into();
/// assert_eq!(promoted, TaiNanos(1_700_000_037_000_000_000));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TaiNanos(pub i128);

/// Continuous nanoseconds in the GPS time scale.
///
/// Uses `i128` to hold the full range of nanoseconds without overflow.
/// Promote from [`GpstSeconds`] via `.into()`, truncate back with
/// [`to_seconds_floor`](Self::to_seconds_floor).
///
/// # Example
///
/// ```
/// use leap_sec::{GpstSeconds, GpstNanos};
///
/// let ns = GpstNanos(1_700_000_018_500_000_000);
/// assert_eq!(format!("{ns}"), "1700000018500000000 GPST");
///
/// let promoted: GpstNanos = GpstSeconds(1_700_000_018).into();
/// assert_eq!(promoted, GpstNanos(1_700_000_018_000_000_000));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GpstNanos(pub i128);

const NANOS_PER_SECOND: i128 = 1_000_000_000;

// ---------------------------------------------------------------------------
// Display — always shows the scale label
// ---------------------------------------------------------------------------

/// Displays as `"{seconds} UTC"` (e.g., `"1700000000 UTC"`).
impl fmt::Display for UtcUnixSeconds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} UTC", self.0)
    }
}

/// Displays as `"{seconds} TAI"` (e.g., `"1700000037 TAI"`).
impl fmt::Display for TaiSeconds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} TAI", self.0)
    }
}

/// Displays as `"{seconds} GPST"` (e.g., `"1700000018 GPST"`).
impl fmt::Display for GpstSeconds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} GPST", self.0)
    }
}

/// Displays as `"{nanoseconds} UTC"` (e.g., `"1700000000500000000 UTC"`).
impl fmt::Display for UtcUnixNanos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} UTC", self.0)
    }
}

/// Displays as `"{nanoseconds} TAI"` (e.g., `"1700000037500000000 TAI"`).
impl fmt::Display for TaiNanos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} TAI", self.0)
    }
}

/// Displays as `"{nanoseconds} GPST"` (e.g., `"1700000018500000000 GPST"`).
impl fmt::Display for GpstNanos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} GPST", self.0)
    }
}

// ---------------------------------------------------------------------------
// From<Seconds> for Nanos — lossless promotion
// ---------------------------------------------------------------------------

/// Lossless promotion: multiplies by 1,000,000,000.
impl From<UtcUnixSeconds> for UtcUnixNanos {
    fn from(s: UtcUnixSeconds) -> Self {
        Self(i128::from(s.0) * NANOS_PER_SECOND)
    }
}

/// Lossless promotion: multiplies by 1,000,000,000.
impl From<TaiSeconds> for TaiNanos {
    fn from(s: TaiSeconds) -> Self {
        Self(i128::from(s.0) * NANOS_PER_SECOND)
    }
}

/// Lossless promotion: multiplies by 1,000,000,000.
impl From<GpstSeconds> for GpstNanos {
    fn from(s: GpstSeconds) -> Self {
        Self(i128::from(s.0) * NANOS_PER_SECOND)
    }
}

// ---------------------------------------------------------------------------
// to_seconds_floor — truncate nanos to whole seconds (floor division)
// ---------------------------------------------------------------------------

impl UtcUnixNanos {
    /// Truncate to whole seconds, rounding toward negative infinity.
    ///
    /// # Example
    ///
    /// ```
    /// use leap_sec::{UtcUnixSeconds, UtcUnixNanos};
    ///
    /// let ns = UtcUnixNanos(1_700_000_000_999_999_999);
    /// assert_eq!(ns.to_seconds_floor(), UtcUnixSeconds(1_700_000_000));
    ///
    /// // Negative values floor correctly
    /// assert_eq!(UtcUnixNanos(-1).to_seconds_floor(), UtcUnixSeconds(-1));
    /// ```
    #[allow(clippy::cast_possible_truncation)]
    pub const fn to_seconds_floor(self) -> UtcUnixSeconds {
        UtcUnixSeconds(floor_div_i128(self.0, NANOS_PER_SECOND) as i64)
    }
}

impl TaiNanos {
    /// Truncate to whole seconds, rounding toward negative infinity.
    ///
    /// # Example
    ///
    /// ```
    /// use leap_sec::{TaiSeconds, TaiNanos};
    ///
    /// let ns = TaiNanos(1_700_000_037_500_000_000);
    /// assert_eq!(ns.to_seconds_floor(), TaiSeconds(1_700_000_037));
    /// ```
    #[allow(clippy::cast_possible_truncation)]
    pub const fn to_seconds_floor(self) -> TaiSeconds {
        TaiSeconds(floor_div_i128(self.0, NANOS_PER_SECOND) as i64)
    }
}

impl GpstNanos {
    /// Truncate to whole seconds, rounding toward negative infinity.
    ///
    /// # Example
    ///
    /// ```
    /// use leap_sec::{GpstSeconds, GpstNanos};
    ///
    /// let ns = GpstNanos(1_700_000_018_500_000_000);
    /// assert_eq!(ns.to_seconds_floor(), GpstSeconds(1_700_000_018));
    /// ```
    #[allow(clippy::cast_possible_truncation)]
    pub const fn to_seconds_floor(self) -> GpstSeconds {
        GpstSeconds(floor_div_i128(self.0, NANOS_PER_SECOND) as i64)
    }
}

/// Floor division for i128 — rounds toward negative infinity.
const fn floor_div_i128(a: i128, b: i128) -> i128 {
    let d = a / b;
    let r = a % b;
    if (r != 0) && ((r ^ b) < 0) { d - 1 } else { d }
}
