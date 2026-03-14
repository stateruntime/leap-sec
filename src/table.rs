//! The leap-second table and all step-based conversions.

use crate::convert::{gpst_to_tai, tai_to_gpst};
use crate::error::Error;
use crate::types::{GpstNanos, GpstSeconds, TaiNanos, TaiSeconds, UtcUnixNanos, UtcUnixSeconds};

/// A single entry in the leap-second table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(unreachable_pub)]
pub struct LeapEntry {
    /// UTC Unix timestamp at which this offset takes effect.
    pub utc_unix: i64,
    /// Cumulative TAI−UTC offset (in seconds) from this point forward.
    pub tai_minus_utc: i32,
}

/// Storage for the leap-second entries.
#[derive(Debug, Clone)]
enum Storage {
    /// A static (compile-time) slice — used by `known()`.
    Static(&'static [LeapEntry]),
    /// A heap-allocated vector — used by the builder.
    #[cfg(feature = "std")]
    Owned(Vec<LeapEntry>),
}

impl Storage {
    fn entries(&self) -> &[LeapEntry] {
        match self {
            Self::Static(s) => s,
            #[cfg(feature = "std")]
            Self::Owned(v) => v,
        }
    }
}

/// An immutable leap-second schedule.
///
/// Use [`LeapSeconds::known()`] to get the built-in table with all historical
/// leap seconds through 2017-01-01. Works in `no_std`, no allocation, deterministic.
///
/// For custom tables (testing, simulation), use
/// [`LeapSeconds::builder()`](Self::builder) (requires `std` feature).
///
/// `LeapSeconds` is `Send + Sync` — safe to share across threads.
/// The `known()` table returns `&'static LeapSeconds`, so it can be used
/// from any thread without cloning.
#[derive(Debug, Clone)]
pub struct LeapSeconds {
    storage: Storage,
    expires_at: Option<i64>,
}

// ---------------------------------------------------------------------------
// The 28 historical leap-second entries
// ---------------------------------------------------------------------------

const KNOWN_TABLE: [LeapEntry; 28] = [
    LeapEntry {
        utc_unix: 63_072_000,
        tai_minus_utc: 10,
    }, // 1972-01-01
    LeapEntry {
        utc_unix: 78_796_800,
        tai_minus_utc: 11,
    }, // 1972-07-01
    LeapEntry {
        utc_unix: 94_694_400,
        tai_minus_utc: 12,
    }, // 1973-01-01
    LeapEntry {
        utc_unix: 126_230_400,
        tai_minus_utc: 13,
    }, // 1974-01-01
    LeapEntry {
        utc_unix: 157_766_400,
        tai_minus_utc: 14,
    }, // 1975-01-01
    LeapEntry {
        utc_unix: 189_302_400,
        tai_minus_utc: 15,
    }, // 1976-01-01
    LeapEntry {
        utc_unix: 220_924_800,
        tai_minus_utc: 16,
    }, // 1977-01-01
    LeapEntry {
        utc_unix: 252_460_800,
        tai_minus_utc: 17,
    }, // 1978-01-01
    LeapEntry {
        utc_unix: 283_996_800,
        tai_minus_utc: 18,
    }, // 1979-01-01
    LeapEntry {
        utc_unix: 315_532_800,
        tai_minus_utc: 19,
    }, // 1980-01-01
    LeapEntry {
        utc_unix: 362_793_600,
        tai_minus_utc: 20,
    }, // 1981-07-01
    LeapEntry {
        utc_unix: 394_329_600,
        tai_minus_utc: 21,
    }, // 1982-07-01
    LeapEntry {
        utc_unix: 425_865_600,
        tai_minus_utc: 22,
    }, // 1983-07-01
    LeapEntry {
        utc_unix: 489_024_000,
        tai_minus_utc: 23,
    }, // 1985-07-01
    LeapEntry {
        utc_unix: 567_993_600,
        tai_minus_utc: 24,
    }, // 1988-01-01
    LeapEntry {
        utc_unix: 631_152_000,
        tai_minus_utc: 25,
    }, // 1990-01-01
    LeapEntry {
        utc_unix: 662_688_000,
        tai_minus_utc: 26,
    }, // 1991-01-01
    LeapEntry {
        utc_unix: 709_948_800,
        tai_minus_utc: 27,
    }, // 1992-07-01
    LeapEntry {
        utc_unix: 741_484_800,
        tai_minus_utc: 28,
    }, // 1993-07-01
    LeapEntry {
        utc_unix: 773_020_800,
        tai_minus_utc: 29,
    }, // 1994-07-01
    LeapEntry {
        utc_unix: 820_454_400,
        tai_minus_utc: 30,
    }, // 1996-01-01
    LeapEntry {
        utc_unix: 867_715_200,
        tai_minus_utc: 31,
    }, // 1997-07-01
    LeapEntry {
        utc_unix: 915_148_800,
        tai_minus_utc: 32,
    }, // 1999-01-01
    LeapEntry {
        utc_unix: 1_136_073_600,
        tai_minus_utc: 33,
    }, // 2006-01-01
    LeapEntry {
        utc_unix: 1_230_768_000,
        tai_minus_utc: 34,
    }, // 2009-01-01
    LeapEntry {
        utc_unix: 1_341_100_800,
        tai_minus_utc: 35,
    }, // 2012-07-01
    LeapEntry {
        utc_unix: 1_435_708_800,
        tai_minus_utc: 36,
    }, // 2015-07-01
    // The last leap second was inserted on 2016-12-31 at 23:59:60 UTC.
    // The new offset (37) takes effect at 2017-01-01 00:00:00 UTC.
    LeapEntry {
        utc_unix: 1_483_228_800,
        tai_minus_utc: 37,
    }, // 2017-01-01
];

static KNOWN: LeapSeconds = LeapSeconds {
    storage: Storage::Static(&KNOWN_TABLE),
    expires_at: None,
};

const NANOS_PER_SECOND: i128 = 1_000_000_000;

impl LeapSeconds {
    /// Returns the built-in table with all historical leap seconds through 2017-01-01.
    ///
    /// Works in `no_std`, requires no allocation, and is fully deterministic.
    /// Timestamps after 2017-01-01 use the last known offset (37) because
    /// no new leap seconds have been inserted since then.
    ///
    /// For custom tables, see [`LeapSeconds::builder()`](Self::builder).
    ///
    /// # Example
    ///
    /// ```
    /// use leap_sec::prelude::*;
    ///
    /// let leaps = LeapSeconds::known();
    /// let tai = leaps.utc_to_tai(UtcUnixSeconds(1_700_000_000)).unwrap();
    /// assert_eq!(tai, TaiSeconds(1_700_000_037));
    /// ```
    pub fn known() -> &'static Self {
        &KNOWN
    }

    /// Create a `LeapSeconds` from owned entries (used by the builder).
    #[cfg(feature = "std")]
    pub(crate) const fn from_owned(entries: Vec<LeapEntry>, expires_at: Option<i64>) -> Self {
        Self {
            storage: Storage::Owned(entries),
            expires_at,
        }
    }

    fn entries(&self) -> &[LeapEntry] {
        self.storage.entries()
    }

    // -----------------------------------------------------------------------
    // UTC → TAI
    // -----------------------------------------------------------------------

    /// Convert a UTC Unix timestamp to TAI seconds.
    ///
    /// # Errors
    ///
    /// Returns [`Error::OutOfRange`] if `utc` is before the first entry in the table.
    ///
    /// # Example
    ///
    /// ```
    /// use leap_sec::prelude::*;
    ///
    /// let leaps = LeapSeconds::known();
    /// let tai = leaps.utc_to_tai(UtcUnixSeconds(1_700_000_000)).unwrap();
    /// assert_eq!(tai, TaiSeconds(1_700_000_037));
    /// ```
    pub fn utc_to_tai(&self, utc: UtcUnixSeconds) -> Result<TaiSeconds, Error> {
        let offset = self.lookup_utc(utc.0)?;
        Ok(TaiSeconds(utc.0 + i64::from(offset)))
    }

    /// Convert UTC Unix nanoseconds to TAI nanoseconds.
    ///
    /// The offset is applied in whole seconds; the sub-second fraction is preserved exactly.
    ///
    /// # Errors
    ///
    /// Returns [`Error::OutOfRange`] if the timestamp is before 1972-01-01.
    ///
    /// # Example
    ///
    /// ```
    /// use leap_sec::prelude::*;
    ///
    /// let leaps = LeapSeconds::known();
    /// let tai = leaps.utc_to_tai_nanos(UtcUnixNanos(1_700_000_000_500_000_000)).unwrap();
    /// assert_eq!(tai, TaiNanos(1_700_000_037_500_000_000));
    /// ```
    pub fn utc_to_tai_nanos(&self, utc: UtcUnixNanos) -> Result<TaiNanos, Error> {
        let sec = utc.to_seconds_floor();
        let offset = self.lookup_utc(sec.0)?;
        Ok(TaiNanos(utc.0 + i128::from(offset) * NANOS_PER_SECOND))
    }

    // -----------------------------------------------------------------------
    // TAI → UTC
    // -----------------------------------------------------------------------

    /// Convert TAI seconds to a UTC Unix timestamp.
    ///
    /// # Errors
    ///
    /// Returns [`Error::OutOfRange`] if `tai` is before the first entry in the table.
    ///
    /// # Example
    ///
    /// ```
    /// use leap_sec::prelude::*;
    ///
    /// let leaps = LeapSeconds::known();
    /// let utc = leaps.tai_to_utc(TaiSeconds(1_700_000_037)).unwrap();
    /// assert_eq!(utc, UtcUnixSeconds(1_700_000_000));
    /// ```
    pub fn tai_to_utc(&self, tai: TaiSeconds) -> Result<UtcUnixSeconds, Error> {
        let offset = self.lookup_tai(tai.0)?;
        Ok(UtcUnixSeconds(tai.0 - i64::from(offset)))
    }

    /// Convert TAI nanoseconds to UTC Unix nanoseconds.
    ///
    /// # Errors
    ///
    /// Returns [`Error::OutOfRange`] if the timestamp is before the table's TAI range.
    ///
    /// # Example
    ///
    /// ```
    /// use leap_sec::prelude::*;
    ///
    /// let leaps = LeapSeconds::known();
    /// let utc = leaps.tai_to_utc_nanos(TaiNanos(1_700_000_037_500_000_000)).unwrap();
    /// assert_eq!(utc, UtcUnixNanos(1_700_000_000_500_000_000));
    /// ```
    pub fn tai_to_utc_nanos(&self, tai: TaiNanos) -> Result<UtcUnixNanos, Error> {
        let sec = tai.to_seconds_floor();
        let offset = self.lookup_tai(sec.0)?;
        Ok(UtcUnixNanos(tai.0 - i128::from(offset) * NANOS_PER_SECOND))
    }

    // -----------------------------------------------------------------------
    // UTC → GPST (composed via TAI)
    // -----------------------------------------------------------------------

    /// Convert a UTC Unix timestamp to GPS Time.
    ///
    /// Composes `utc_to_tai` then `tai_to_gpst` (TAI − 19).
    ///
    /// # Errors
    ///
    /// Returns [`Error::OutOfRange`] if the timestamp is before 1972-01-01.
    ///
    /// # Example
    ///
    /// ```
    /// use leap_sec::prelude::*;
    ///
    /// let leaps = LeapSeconds::known();
    /// let gpst = leaps.utc_to_gpst(UtcUnixSeconds(1_700_000_000)).unwrap();
    /// assert_eq!(gpst, GpstSeconds(1_700_000_018));
    /// ```
    pub fn utc_to_gpst(&self, utc: UtcUnixSeconds) -> Result<GpstSeconds, Error> {
        self.utc_to_tai(utc).map(tai_to_gpst)
    }

    /// Convert UTC Unix nanoseconds to GPST nanoseconds.
    ///
    /// # Errors
    ///
    /// Returns [`Error::OutOfRange`] if the timestamp is before 1972-01-01.
    ///
    /// # Example
    ///
    /// ```
    /// use leap_sec::prelude::*;
    ///
    /// let leaps = LeapSeconds::known();
    /// let gpst = leaps.utc_to_gpst_nanos(UtcUnixNanos(1_700_000_000_500_000_000)).unwrap();
    /// assert_eq!(gpst, GpstNanos(1_700_000_018_500_000_000));
    /// ```
    pub fn utc_to_gpst_nanos(&self, utc: UtcUnixNanos) -> Result<GpstNanos, Error> {
        self.utc_to_tai_nanos(utc)
            .map(crate::convert::tai_to_gpst_nanos)
    }

    // -----------------------------------------------------------------------
    // GPST → UTC (composed via TAI)
    // -----------------------------------------------------------------------

    /// Convert GPS Time to a UTC Unix timestamp.
    ///
    /// Composes `gpst_to_tai` (GPST + 19) then `tai_to_utc`.
    ///
    /// # Errors
    ///
    /// Returns [`Error::OutOfRange`] if the resulting TAI is before the table's range.
    ///
    /// # Example
    ///
    /// ```
    /// use leap_sec::prelude::*;
    ///
    /// let leaps = LeapSeconds::known();
    /// let utc = leaps.gpst_to_utc(GpstSeconds(1_700_000_018)).unwrap();
    /// assert_eq!(utc, UtcUnixSeconds(1_700_000_000));
    /// ```
    pub fn gpst_to_utc(&self, gpst: GpstSeconds) -> Result<UtcUnixSeconds, Error> {
        self.tai_to_utc(gpst_to_tai(gpst))
    }

    /// Convert GPST nanoseconds to UTC Unix nanoseconds.
    ///
    /// # Errors
    ///
    /// Returns [`Error::OutOfRange`] if the resulting TAI is before the table's range.
    ///
    /// # Example
    ///
    /// ```
    /// use leap_sec::prelude::*;
    ///
    /// let leaps = LeapSeconds::known();
    /// let utc = leaps.gpst_to_utc_nanos(GpstNanos(1_700_000_018_500_000_000)).unwrap();
    /// assert_eq!(utc, UtcUnixNanos(1_700_000_000_500_000_000));
    /// ```
    pub fn gpst_to_utc_nanos(&self, gpst: GpstNanos) -> Result<UtcUnixNanos, Error> {
        self.tai_to_utc_nanos(crate::convert::gpst_to_tai_nanos(gpst))
    }

    // -----------------------------------------------------------------------
    // Offset queries
    // -----------------------------------------------------------------------

    /// Get the TAI−UTC offset at a given UTC instant.
    ///
    /// # Errors
    ///
    /// Returns [`Error::OutOfRange`] if the timestamp is before 1972-01-01.
    ///
    /// # Example
    ///
    /// ```
    /// use leap_sec::prelude::*;
    ///
    /// let leaps = LeapSeconds::known();
    /// assert_eq!(leaps.tai_utc_offset(UtcUnixSeconds(1_700_000_000)).unwrap(), 37);
    /// assert_eq!(leaps.tai_utc_offset(UtcUnixSeconds(63_072_000)).unwrap(), 10);
    /// ```
    pub fn tai_utc_offset(&self, utc: UtcUnixSeconds) -> Result<i32, Error> {
        self.lookup_utc(utc.0)
    }

    /// Get the TAI−UTC offset at a given TAI instant.
    ///
    /// # Errors
    ///
    /// Returns [`Error::OutOfRange`] if the TAI timestamp is before the table's range.
    ///
    /// # Example
    ///
    /// ```
    /// use leap_sec::prelude::*;
    ///
    /// let leaps = LeapSeconds::known();
    /// assert_eq!(leaps.tai_utc_offset_at_tai(TaiSeconds(1_700_000_037)).unwrap(), 37);
    /// ```
    pub fn tai_utc_offset_at_tai(&self, tai: TaiSeconds) -> Result<i32, Error> {
        self.lookup_tai(tai.0)
    }

    // -----------------------------------------------------------------------
    // Leap-second detection
    // -----------------------------------------------------------------------

    /// Returns `true` if `utc` falls exactly on a positive leap-second insertion.
    ///
    /// At such an instant the POSIX timestamp is ambiguous: the UTC wall clock
    /// reads `23:59:60` but POSIX folds it to the same value as `00:00:00`.
    ///
    /// Returns `false` for:
    /// - The initial 1972-01-01 epoch (offset = 10) — not an insertion, it is
    ///   the starting offset of the modern UTC system.
    /// - Negative leap seconds (where the offset *decreases*) — there is no
    ///   extra second, so no ambiguity. No negative leap second has ever been
    ///   applied, but the table format supports them.
    ///
    /// # Example
    ///
    /// ```
    /// use leap_sec::prelude::*;
    ///
    /// let leaps = LeapSeconds::known();
    ///
    /// // 2017-01-01 — the last leap second insertion (2016-12-31 23:59:60)
    /// assert!(leaps.is_during_leap_second(UtcUnixSeconds(1_483_228_800)));
    ///
    /// // A normal timestamp — not during a leap second
    /// assert!(!leaps.is_during_leap_second(UtcUnixSeconds(1_700_000_000)));
    /// ```
    pub fn is_during_leap_second(&self, utc: UtcUnixSeconds) -> bool {
        let entries = self.entries();

        // Binary search for the timestamp.
        let Ok(idx) = entries.binary_search_by_key(&utc.0, |e| e.utc_unix) else {
            return false; // Not an exact entry boundary.
        };

        // Skip the first entry (the initial epoch, not an insertion).
        // Check that the offset *increased* (positive leap second only).
        idx > 0 && entries[idx].tai_minus_utc > entries[idx - 1].tai_minus_utc
    }

    // -----------------------------------------------------------------------
    // Table inspection
    // -----------------------------------------------------------------------

    /// Returns the valid range of this table as `(first_entry, last_entry)`.
    ///
    /// # Example
    ///
    /// ```
    /// use leap_sec::prelude::*;
    ///
    /// let leaps = LeapSeconds::known();
    /// let (start, end) = leaps.valid_range();
    /// assert_eq!(start, UtcUnixSeconds(63_072_000));    // 1972-01-01
    /// assert_eq!(end, UtcUnixSeconds(1_483_228_800));   // 2017-01-01
    /// ```
    pub fn valid_range(&self) -> (UtcUnixSeconds, UtcUnixSeconds) {
        let entries = self.entries();
        (
            UtcUnixSeconds(entries[0].utc_unix),
            UtcUnixSeconds(entries[entries.len() - 1].utc_unix),
        )
    }

    /// Returns `false` for the built-in `known()` table.
    ///
    /// For tables with an expiration timestamp, this would return `true`
    /// if the table has expired. In v0.1 there is no clock access — this
    /// simply checks whether an expiration is set.
    pub const fn is_expired(&self) -> bool {
        false
    }

    /// Returns the expiration timestamp, if one was set.
    ///
    /// The built-in `known()` table returns `None` (it has no expiration concept).
    /// Tables constructed via the builder may have an expiration set.
    ///
    /// # Example
    ///
    /// ```
    /// use leap_sec::prelude::*;
    ///
    /// assert_eq!(LeapSeconds::known().expires_at(), None);
    /// ```
    pub fn expires_at(&self) -> Option<UtcUnixSeconds> {
        self.expires_at.map(UtcUnixSeconds)
    }

    /// Returns the most recent leap-second entry: `(effective_utc, tai_minus_utc)`.
    ///
    /// For the built-in table this is `(2017-01-01, 37)`.
    ///
    /// # Example
    ///
    /// ```
    /// use leap_sec::prelude::*;
    ///
    /// let leaps = LeapSeconds::known();
    /// let (date, offset) = leaps.latest_entry();
    /// assert_eq!(date, UtcUnixSeconds(1_483_228_800));
    /// assert_eq!(offset, 37);
    /// ```
    pub fn latest_entry(&self) -> (UtcUnixSeconds, i32) {
        let entries = self.entries();
        let last = entries[entries.len() - 1];
        (UtcUnixSeconds(last.utc_unix), last.tai_minus_utc)
    }

    // -----------------------------------------------------------------------
    // Internal lookup helpers
    // -----------------------------------------------------------------------

    /// Binary search for the TAI−UTC offset at a UTC Unix timestamp.
    fn lookup_utc(&self, utc: i64) -> Result<i32, Error> {
        let entries = self.entries();

        if utc < entries[0].utc_unix {
            return Err(Error::OutOfRange {
                requested: utc,
                valid_start: entries[0].utc_unix,
                valid_end: entries[entries.len() - 1].utc_unix,
            });
        }

        // Binary search: find the last entry whose utc_unix <= utc.
        let idx = match entries.binary_search_by_key(&utc, |e| e.utc_unix) {
            Ok(i) => i,
            Err(i) => i - 1, // i > 0 because we checked utc >= entries[0] above
        };

        Ok(entries[idx].tai_minus_utc)
    }

    /// Binary search for the TAI−UTC offset at a TAI timestamp.
    ///
    /// Each entry's TAI boundary is `utc_unix + tai_minus_utc`.
    fn lookup_tai(&self, tai: i64) -> Result<i32, Error> {
        let entries = self.entries();

        let first_tai = entries[0].utc_unix + i64::from(entries[0].tai_minus_utc);
        if tai < first_tai {
            return Err(Error::OutOfRange {
                requested: tai,
                valid_start: first_tai,
                valid_end: entries[entries.len() - 1].utc_unix
                    + i64::from(entries[entries.len() - 1].tai_minus_utc),
            });
        }

        // Binary search in TAI space: entry boundary is utc_unix + tai_minus_utc.
        let mut lo = 0;
        let mut hi = entries.len();
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            let tai_boundary = entries[mid].utc_unix + i64::from(entries[mid].tai_minus_utc);
            if tai_boundary <= tai {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }

        // lo is the first entry whose TAI boundary > tai, so we want lo - 1.
        let idx = if lo > 0 { lo - 1 } else { 0 };
        Ok(entries[idx].tai_minus_utc)
    }
}

// ---------------------------------------------------------------------------
// Builder support (re-export the entry type for builder module)
// ---------------------------------------------------------------------------

#[cfg(feature = "std")]
#[allow(unreachable_pub)]
pub use self::LeapEntry as LeapEntryInner;
