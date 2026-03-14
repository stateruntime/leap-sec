//! # leap-sec
//!
//! Leap-second handling and continuous time mappings for flight and space systems.
//!
//! This crate provides type-safe conversions between UTC, TAI, and GPS time scales
//! using an explicit leap-second table. The API is designed so that you cannot
//! accidentally mix time scales at call sites.
//!
//! # Quick Start
//!
//! ```
//! use leap_sec::prelude::*;
//!
//! let leaps = LeapSeconds::known();
//!
//! // Convert UTC to TAI
//! let utc = UtcUnixSeconds(1_700_000_000);
//! let tai = leaps.utc_to_tai(utc).unwrap();
//! assert_eq!(tai, TaiSeconds(1_700_000_037));
//!
//! // Roundtrip
//! let back = leaps.tai_to_utc(tai).unwrap();
//! assert_eq!(back, utc);
//! ```
//!
//! # Leap-Second History
//!
//! Since 1972, the IERS has inserted 27 leap seconds. Each insertion increases the
//! TAI−UTC offset by 1 second. The most recent insertion was on **2016-12-31**
//! at 23:59:60 UTC, making the new offset (TAI−UTC = 37s) effective from
//! **2017-01-01** 00:00:00 UTC. No new leap seconds have been inserted since.
//!
//! The built-in [`LeapSeconds::known()`] table contains all 28 entries: the initial
//! 1972-01-01 epoch (offset 10) plus the 27 subsequent insertions.
//!
//! # The Future of UTC (2030s)
//!
//! In 2022, the 27th CGPM adopted [Resolution 4](https://www.bipm.org/documents/20126/64894729/Resolutions27thCGPM-EN.pdf)
//! on the future of UTC: the plan is to increase the maximum allowed value of
//! `|UT1−UTC|` (currently kept within ±0.9s using leap seconds) at a date to be
//! determined, with key decisions targeted by or before 2035.
//!
//! In practice, this is widely understood as the path to ending leap seconds.
//! This crate treats leap seconds as explicit table data, so if they stop being
//! inserted, the table stops growing and historical conversions remain correct.
//!
//! # Negative Leap Seconds
//!
//! The ITU-R Recommendation TF.460 allows *negative* leap seconds (where the
//! TAI−UTC offset decreases by 1 and UTC skips from `23:59:58` directly to
//! `00:00:00`). No negative leap second has ever been applied — all 27
//! historical insertions have been positive.
//!
//! The table format and conversion logic handle them correctly:
//! - A table entry where the new offset is less than the previous offset
//!   represents a negative leap second.
//! - [`LeapSeconds::is_during_leap_second()`] returns `false` for negative leap
//!   seconds (there is no extra second; instead a second is removed).
//! - Roundtrip `utc_to_tai` → `tai_to_utc` remains correct.
//!
//! The custom table builder accepts negative entries for testing.

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]

mod convert;
mod error;
mod table;
mod types;

#[cfg(feature = "std")]
mod builder;

// Re-export free functions at crate root.
pub use convert::{gpst_to_tai, gpst_to_tai_nanos, tai_to_gpst, tai_to_gpst_nanos};
pub use error::Error;
pub use table::LeapSeconds;
pub use types::{GpstNanos, GpstSeconds, TaiNanos, TaiSeconds, UtcUnixNanos, UtcUnixSeconds};

#[cfg(feature = "std")]
pub use builder::LeapSecondsBuilder;

/// Prelude for convenient glob imports.
///
/// Brings all public types, free functions, and the error type into scope:
///
/// - **Table:** [`LeapSeconds`]
/// - **Seconds types:** [`UtcUnixSeconds`], [`TaiSeconds`], [`GpstSeconds`]
/// - **Nanos types:** [`UtcUnixNanos`], [`TaiNanos`], [`GpstNanos`]
/// - **Free functions:** [`tai_to_gpst`], [`gpst_to_tai`], [`tai_to_gpst_nanos`], [`gpst_to_tai_nanos`]
/// - **Error:** [`Error`]
/// - **Builder:** [`LeapSecondsBuilder`] (requires `std` feature)
///
/// ```
/// use leap_sec::prelude::*;
///
/// let leaps = LeapSeconds::known();
/// let tai = leaps.utc_to_tai(UtcUnixSeconds(1_700_000_000)).unwrap();
/// assert_eq!(tai, TaiSeconds(1_700_000_037));
/// ```
pub mod prelude {
    pub use crate::convert::{gpst_to_tai, gpst_to_tai_nanos, tai_to_gpst, tai_to_gpst_nanos};
    pub use crate::error::Error;
    pub use crate::table::LeapSeconds;
    pub use crate::types::{
        GpstNanos, GpstSeconds, TaiNanos, TaiSeconds, UtcUnixNanos, UtcUnixSeconds,
    };

    #[cfg(feature = "std")]
    pub use crate::builder::LeapSecondsBuilder;
}
