//! Error types for leap-second conversions.

use core::fmt;

/// Errors that can occur during leap-second conversions or table construction.
///
/// All conversion methods on [`LeapSeconds`](crate::LeapSeconds) return
/// `Result<T, Error>`. Pattern-match to handle specific cases:
///
/// # Example
///
/// ```
/// use leap_sec::prelude::*;
///
/// let leaps = LeapSeconds::known();
///
/// match leaps.utc_to_tai(UtcUnixSeconds(0)) {
///     Ok(tai) => println!("TAI: {tai}"),
///     Err(Error::OutOfRange { requested, valid_start, .. }) => {
///         println!("{requested} is before {valid_start}");
///     }
///     Err(e) => println!("other error: {e}"),
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// The requested timestamp is before the first entry in the leap-second table.
    ///
    /// Returned by [`LeapSeconds::utc_to_tai`](crate::LeapSeconds::utc_to_tai),
    /// [`LeapSeconds::tai_to_utc`](crate::LeapSeconds::tai_to_utc), and all
    /// other conversion and offset methods when the input is outside the table's
    /// valid range.
    OutOfRange {
        /// The timestamp that was requested.
        requested: i64,
        /// The earliest valid timestamp in the table.
        valid_start: i64,
        /// The latest entry timestamp in the table.
        valid_end: i64,
    },
    /// The leap-second table has expired.
    ///
    /// Reserved for future use when tables parsed from IERS files carry
    /// expiration timestamps. The built-in [`LeapSeconds::known()`](crate::LeapSeconds::known)
    /// table never produces this error.
    TableExpired {
        /// When the table expires, as a UTC Unix timestamp.
        expires_at: i64,
    },
    /// The table data is invalid (used during builder validation).
    ///
    /// Returned by [`LeapSecondsBuilder::build()`](crate::LeapSecondsBuilder::build)
    /// when the table is empty or timestamps are not monotonically increasing.
    InvalidTable {
        /// A description of what is wrong with the table.
        detail: &'static str,
    },
}

/// Formats the error with a human-readable message.
///
/// - `OutOfRange`: `"timestamp {requested} is outside the leap-second table range [{start}, {end}]"`
/// - `TableExpired`: `"leap-second table expired at {expires_at}; load a newer table or update the crate"`
/// - `InvalidTable`: `"invalid leap-second table: {detail}"`
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutOfRange {
                requested,
                valid_start,
                valid_end,
            } => write!(
                f,
                "timestamp {requested} is outside the leap-second table range \
                 [{valid_start}, {valid_end}]"
            ),
            Self::TableExpired { expires_at } => {
                write!(
                    f,
                    "leap-second table expired at {expires_at}; \
                     load a newer table or update the crate"
                )
            }
            Self::InvalidTable { detail } => {
                write!(f, "invalid leap-second table: {detail}")
            }
        }
    }
}

/// Enables use as `Box<dyn std::error::Error>` and in error-handling chains.
///
/// Available only with the `std` feature (enabled by default).
#[cfg(feature = "std")]
impl std::error::Error for Error {}
