//! Builder for custom leap-second tables.
//!
//! Useful for testing with synthetic schedules or for constructing tables
//! from parsed data sources.

use crate::error::Error;
use crate::table::{LeapEntryInner, LeapSeconds};
use crate::types::UtcUnixSeconds;

/// A builder for constructing custom [`LeapSeconds`] tables.
///
/// # Example
///
/// ```
/// use leap_sec::prelude::*;
///
/// let table = LeapSeconds::builder()
///     .add(UtcUnixSeconds(63_072_000), 10)   // 1972-01-01
///     .add(UtcUnixSeconds(78_796_800), 11)   // 1972-07-01
///     .build()
///     .unwrap();
///
/// let tai = table.utc_to_tai(UtcUnixSeconds(70_000_000)).unwrap();
/// assert_eq!(tai, TaiSeconds(70_000_010));
/// ```
#[derive(Debug, Clone)]
pub struct LeapSecondsBuilder {
    entries: Vec<LeapEntryInner>,
    expires_at: Option<i64>,
}

impl LeapSecondsBuilder {
    /// Create a new empty builder.
    pub const fn new() -> Self {
        Self {
            entries: Vec::new(),
            expires_at: None,
        }
    }

    /// Add a leap-second entry.
    ///
    /// `utc` is the UTC Unix timestamp at which the offset takes effect.
    /// `tai_minus_utc` is the cumulative TAI−UTC offset from this point forward.
    #[must_use]
    pub fn add(mut self, utc: UtcUnixSeconds, tai_minus_utc: i32) -> Self {
        self.entries.push(LeapEntryInner {
            utc_unix: utc.0,
            tai_minus_utc,
        });
        self
    }

    /// Set an expiration timestamp for the table.
    #[must_use]
    pub const fn expires_at(mut self, at: UtcUnixSeconds) -> Self {
        self.expires_at = Some(at.0);
        self
    }

    /// Build the leap-second table.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidTable`] if:
    /// - The table is empty
    /// - Timestamps are not monotonically increasing
    pub fn build(self) -> Result<LeapSeconds, Error> {
        if self.entries.is_empty() {
            return Err(Error::InvalidTable {
                detail: "table must contain at least one entry",
            });
        }

        for w in self.entries.windows(2) {
            if w[1].utc_unix <= w[0].utc_unix {
                return Err(Error::InvalidTable {
                    detail: "timestamps must be monotonically increasing",
                });
            }
        }

        Ok(LeapSeconds::from_owned(self.entries, self.expires_at))
    }
}

impl Default for LeapSecondsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl LeapSeconds {
    /// Create a builder for constructing a custom leap-second table.
    pub const fn builder() -> LeapSecondsBuilder {
        LeapSecondsBuilder::new()
    }
}
