//! Free functions for TAI ↔ GPST conversions.
//!
//! TAI and GPST differ by a constant 19 seconds — no leap-second table is needed.
//! These functions are always valid regardless of date range.

use crate::types::{GpstNanos, GpstSeconds, TaiNanos, TaiSeconds};

/// The constant offset between TAI and GPST in seconds.
const TAI_GPST_OFFSET: i64 = 19;

/// The constant offset between TAI and GPST in nanoseconds.
const TAI_GPST_OFFSET_NANOS: i128 = 19_000_000_000;

/// Convert TAI to GPS Time. `GPST = TAI − 19s`. Always exact.
///
/// See also: [`gpst_to_tai`] (inverse), [`LeapSeconds::utc_to_gpst`](crate::LeapSeconds::utc_to_gpst) (from UTC).
///
/// # Example
///
/// ```
/// use leap_sec::{tai_to_gpst, TaiSeconds, GpstSeconds};
///
/// let gpst = tai_to_gpst(TaiSeconds(1_700_000_037));
/// assert_eq!(gpst, GpstSeconds(1_700_000_018));
/// ```
pub const fn tai_to_gpst(tai: TaiSeconds) -> GpstSeconds {
    GpstSeconds(tai.0 - TAI_GPST_OFFSET)
}

/// Convert GPS Time to TAI. `TAI = GPST + 19s`. Always exact.
///
/// See also: [`tai_to_gpst`] (inverse), [`LeapSeconds::gpst_to_utc`](crate::LeapSeconds::gpst_to_utc) (to UTC).
///
/// # Example
///
/// ```
/// use leap_sec::{gpst_to_tai, TaiSeconds, GpstSeconds};
///
/// let tai = gpst_to_tai(GpstSeconds(1_700_000_018));
/// assert_eq!(tai, TaiSeconds(1_700_000_037));
/// ```
pub const fn gpst_to_tai(gpst: GpstSeconds) -> TaiSeconds {
    TaiSeconds(gpst.0 + TAI_GPST_OFFSET)
}

/// Convert TAI nanoseconds to GPST nanoseconds. Always exact.
///
/// See also: [`gpst_to_tai_nanos`] (inverse).
///
/// # Example
///
/// ```
/// use leap_sec::{tai_to_gpst_nanos, TaiNanos, GpstNanos};
///
/// let gpst = tai_to_gpst_nanos(TaiNanos(1_700_000_037_000_000_000));
/// assert_eq!(gpst, GpstNanos(1_700_000_018_000_000_000));
/// ```
pub const fn tai_to_gpst_nanos(tai: TaiNanos) -> GpstNanos {
    GpstNanos(tai.0 - TAI_GPST_OFFSET_NANOS)
}

/// Convert GPST nanoseconds to TAI nanoseconds. Always exact.
///
/// See also: [`tai_to_gpst_nanos`] (inverse).
///
/// # Example
///
/// ```
/// use leap_sec::{gpst_to_tai_nanos, TaiNanos, GpstNanos};
///
/// let tai = gpst_to_tai_nanos(GpstNanos(1_700_000_018_000_000_000));
/// assert_eq!(tai, TaiNanos(1_700_000_037_000_000_000));
/// ```
pub const fn gpst_to_tai_nanos(gpst: GpstNanos) -> TaiNanos {
    TaiNanos(gpst.0 + TAI_GPST_OFFSET_NANOS)
}
