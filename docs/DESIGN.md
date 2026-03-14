# Design

This document captures the intended API shape for `leap-sec`. It is a design contract, not an implementation.

## Vocabulary

| Term | Meaning |
|------|---------|
| **UTC** | Coordinated Universal Time — civil time scale with leap seconds (discontinuous). |
| **TAI** | International Atomic Time — continuous, monotonic seconds counter. As of 2017-01-01 (most recent leap second insertion to date), TAI−UTC = 37s. |
| **GPST** | GPS Time — continuous, offset from TAI by exactly 19s. As of 2017-01-01, GPS−UTC = 18s (because TAI−GPS = 19s by definition). |
| **Leap-second table** | A schedule of all leap-second insertions (dates + cumulative TAI−UTC offset), used to convert between UTC and continuous scales. |
| **Smear** | A policy that spreads a leap second over a time window instead of inserting a discrete extra second. |
| **Step** | The real, physical leap-second insertion: `23:59:59 → 23:59:60 → 00:00:00`. |

## How Leap Seconds Work

Since 1972, the IERS has inserted leap seconds at the end of June 30 or December 31 when the difference between UTC and Earth rotation (UT1) approaches 0.9 seconds. Each insertion increases the TAI−UTC offset by 1 second.

**Complete leap-second history:**

| Date | TAI−UTC after | Date | TAI−UTC after |
|------|--------------|------|--------------|
| 1972-01-01 | 10s | 1994-07-01 | 29s |
| 1972-07-01 | 11s | 1996-01-01 | 30s |
| 1973-01-01 | 12s | 1997-07-01 | 31s |
| 1974-01-01 | 13s | 1999-01-01 | 32s |
| 1975-01-01 | 14s | 2006-01-01 | 33s |
| 1976-01-01 | 15s | 2009-01-01 | 34s |
| 1977-01-01 | 16s | 2012-07-01 | 35s |
| 1978-01-01 | 17s | 2015-07-01 | 36s |
| 1979-01-01 | 18s | 2017-01-01 | 37s |
| 1980-01-01 | 19s | | |
| 1981-07-01 | 20s | *No more since* | |
| 1982-07-01 | 21s | *2017-01-01.* | |
| 1983-07-01 | 22s | | |
| 1985-07-01 | 23s | | |
| 1988-01-01 | 24s | | |
| 1990-01-01 | 25s | | |
| 1991-01-01 | 26s | | |
| 1992-07-01 | 27s | | |
| 1993-07-01 | 28s | | |

### The future of UTC (2030s)

In 2022, the 27th CGPM adopted Resolution 4 on the future of UTC: the plan is to increase the maximum
allowed value of `|UT1−UTC|` (currently kept within ±0.9s using leap seconds) at a date to be determined,
with key decisions targeted by or before 2035.

This crate treats leap seconds as explicit table data, so if leap seconds stop being inserted, the table
stops growing and historical conversions remain correct.

## Core Types (Draft)

### LeapSeconds

```rust
/// An immutable leap-second schedule.
pub struct LeapSeconds { /* sorted list of (utc_timestamp, tai_minus_utc) pairs */ }

impl LeapSeconds {
    /// Returns the built-in table with all historical leap seconds through 2017-01-01.
    /// Works in no_std, no allocation, deterministic.
    pub fn known() -> &'static Self;

    /// Parse an IERS/NTP leap-seconds.list file (feature-gated, requires std).
    ///
    /// > **Not yet implemented** — planned for v0.2. See [ROADMAP.md](ROADMAP.md).
    pub fn from_iers_list(data: &str) -> Result<Self, Error>;

    /// The valid range of this table.
    pub fn valid_range(&self) -> (UtcUnixSeconds, UtcUnixSeconds);

    /// Whether this table has expired (last entry is in the past).
    pub fn is_expired(&self) -> bool;
}
```

### Time Newtypes

#### Second-precision (primary)

```rust
/// Unix-like seconds count in the UTC scale.
/// Note: does NOT represent the leap second itself (23:59:60).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct UtcUnixSeconds(pub i64);

/// Continuous seconds count in the TAI scale (no leap seconds).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TaiSeconds(pub i64);

/// Continuous seconds count in the GPS time scale (TAI − 19s).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct GpstSeconds(pub i64);
```

#### Sub-second precision variants (nanoseconds)

Sub-second precision is required for GNSS, spacecraft telemetry, and any domain
where timestamps carry fractional seconds. The nanosecond variants use `i128` to
hold the full range of Unix nanoseconds without overflow.

```rust
/// Unix-epoch nanoseconds in the UTC scale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct UtcUnixNanos(pub i128);

/// Continuous nanoseconds in the TAI scale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TaiNanos(pub i128);

/// Continuous nanoseconds in the GPS time scale (TAI − 19s).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct GpstNanos(pub i128);
```

Both precisions are first-class. All conversion methods accept either:

```rust
impl LeapSeconds {
    pub fn utc_to_tai(&self, utc: UtcUnixSeconds) -> Result<TaiSeconds, Error>;
    pub fn utc_to_tai_nanos(&self, utc: UtcUnixNanos) -> Result<TaiNanos, Error>;
}
```

Lossless conversions between precisions:

```rust
impl From<UtcUnixSeconds> for UtcUnixNanos { /* seconds * 1_000_000_000 */ }
impl UtcUnixNanos {
    pub fn to_seconds_floor(&self) -> UtcUnixSeconds;
}
// Same for TaiSeconds/TaiNanos, GpstSeconds/GpstNanos.
```

#### Display trait

All newtypes implement `Display` with an explicit scale label so the time scale
is always visible in logs and debug output:

```rust
impl fmt::Display for UtcUnixSeconds {
    // "1704067200 UTC"
}
impl fmt::Display for TaiSeconds {
    // "1704067237 TAI"
}
impl fmt::Display for GpstSeconds {
    // "1704067218 GPST"
}
// Nano variants: "1704067200000000000 UTC", "1704067237000000000 TAI", etc.
```

## Conversions (Draft)

### Step-Based (default, correct)

```rust
impl LeapSeconds {
    /// Convert UTC to TAI using the leap-second table.
    /// Returns an error if the timestamp is outside the table's valid range.
    pub fn utc_to_tai(&self, utc: UtcUnixSeconds) -> Result<TaiSeconds, Error>;

    /// Convert TAI to UTC using the leap-second table.
    pub fn tai_to_utc(&self, tai: TaiSeconds) -> Result<UtcUnixSeconds, Error>;

    /// Convert UTC to GPS time (TAI − 19s).
    pub fn utc_to_gpst(&self, utc: UtcUnixSeconds) -> Result<GpstSeconds, Error>;

    /// Convert GPS time to UTC.
    pub fn gpst_to_utc(&self, gpst: GpstSeconds) -> Result<UtcUnixSeconds, Error>;

    /// Get the TAI−UTC offset at a given UTC instant.
    pub fn tai_utc_offset(&self, utc: UtcUnixSeconds) -> Result<i32, Error>;

    /// Get the TAI−UTC offset at a given TAI instant.
    pub fn tai_utc_offset_at_tai(&self, tai: TaiSeconds) -> Result<i32, Error>;
}
```

### Leap-Second Boundary Detection

During a positive leap-second insertion the UTC clock reads `23:59:60` — the
61st second. In POSIX-style timestamps this second is ambiguous: the timestamp
for `23:59:60` has the same integer value as the following `00:00:00`. The
library makes this observable:

```rust
impl LeapSeconds {
    /// Returns `true` if the given UTC timestamp falls exactly on a
    /// leap-second insertion (the 61st second, 23:59:60).
    ///
    /// At such an instant, the POSIX timestamp is ambiguous: two
    /// distinct UTC wall-clock readings map to the same integer.
    /// Callers can use this to log, flag, or special-case that instant.
    pub fn is_during_leap_second(&self, utc: UtcUnixSeconds) -> bool;
}
```

When `is_during_leap_second` returns `true`, `utc_to_tai` still succeeds — it
returns the TAI instant *after* the insertion (matching the POSIX convention of
folding the extra second into the next day). The flag exists so callers who care
about the ambiguity can detect and handle it.

### Table Inspection

```rust
impl LeapSeconds {
    /// Returns the expiration timestamp from the IERS file, if one was
    /// parsed. The built-in `known()` table returns `None` (it has no
    /// expiration concept — it simply contains all historical entries).
    pub fn expires_at(&self) -> Option<UtcUnixSeconds>;

    /// Returns the most recent leap-second entry: `(effective_utc, tai_minus_utc)`.
    /// For the built-in table this is `(2017-01-01, 37)`.
    pub fn latest_entry(&self) -> (UtcUnixSeconds, i32);
}
```

### TAI to/from GPST Direct Conversions

TAI and GPST differ by a constant 19 seconds — no leap-second table is needed.
These free functions are always valid regardless of table state or date range:

```rust
/// Convert TAI to GPS Time. GPST = TAI − 19s. Always exact.
pub fn tai_to_gpst(tai: TaiSeconds) -> GpstSeconds;

/// Convert GPS Time to TAI. TAI = GPST + 19s. Always exact.
pub fn gpst_to_tai(gpst: GpstSeconds) -> TaiSeconds;

/// Nanosecond variants.
pub fn tai_to_gpst_nanos(tai: TaiNanos) -> GpstNanos;
pub fn gpst_to_tai_nanos(gpst: GpstNanos) -> TaiNanos;
```

These are standalone functions (not methods on `LeapSeconds`) because the offset
is a physical constant, not a table lookup.

### Relaxed / Extrapolation Mode

> **Not yet implemented** — planned for v0.2. See [ROADMAP.md](ROADMAP.md).

For simulations and scenarios that extend beyond the table's expiration, a
relaxed policy suppresses `TableExpired` errors and assumes the last known
TAI−UTC offset continues indefinitely:

```rust
/// Controls behavior when the table is expired.
pub enum ExpirationPolicy {
    /// Default — return `Error::TableExpired` for timestamps past expiration.
    Strict,
    /// Assume the last known offset continues. **For simulation only.**
    /// Real-world UTC may have gained additional leap seconds since the
    /// table was published, so results are approximate.
    Extrapolate,
}

impl LeapSeconds {
    /// Create a wrapper that uses the given expiration policy.
    pub fn with_policy(&self, policy: ExpirationPolicy) -> LeapSecondsView<'_>;
}

impl LeapSecondsView<'_> {
    // Provides the same conversion methods as LeapSeconds.
    pub fn utc_to_tai(&self, utc: UtcUnixSeconds) -> Result<TaiSeconds, Error>;
    pub fn tai_to_utc(&self, tai: TaiSeconds) -> Result<UtcUnixSeconds, Error>;
    // ...
}
```

> **Warning:** Extrapolation mode is explicitly *not* suitable for operational
> systems. It exists for offline simulation, test-data generation, and
> environments where an updated table is unavailable. The guardrail section
> below still applies — the mode is opt-in and loudly documented.

### Bulk / Vectorized Conversions

> **Not yet implemented** — planned for v0.3. See [ROADMAP.md](ROADMAP.md).

Feature-gated on `std` or `alloc`. Intended for v0.3+:

```rust
#[cfg(feature = "std")]
impl LeapSeconds {
    /// Convert a batch of UTC timestamps to TAI in one call.
    /// Returns an error at the first out-of-range timestamp.
    pub fn utc_to_tai_batch(&self, utc: &[UtcUnixSeconds]) -> Result<Vec<TaiSeconds>, Error>;

    /// Convert a batch of TAI timestamps to UTC.
    pub fn tai_to_utc_batch(&self, tai: &[TaiSeconds]) -> Result<Vec<UtcUnixSeconds>, Error>;
}
```

Batch methods amortize table-lookup overhead when processing large datasets
(telemetry archives, RINEX files, etc.).

### Smear-Based (opt-in)

> **Not yet implemented** — planned for v0.2. See [ROADMAP.md](ROADMAP.md).

```rust
/// A smear policy defines how to spread a leap second over a time window.
pub enum SmearPolicy {
    /// Linear smear over a 24-hour window centered on midnight (Google-style).
    Linear24h,
    /// Linear smear over a custom window.
    LinearCustom { window_seconds: u32 },
}

impl LeapSeconds {
    /// Convert UTC to smeared time using the given policy.
    /// The output is a continuous seconds count that avoids the step.
    pub fn utc_to_smeared(&self, utc: UtcUnixSeconds, policy: SmearPolicy)
        -> Result<f64, Error>;

    /// Convert smeared time back to UTC.
    pub fn smeared_to_utc(&self, smeared: f64, policy: SmearPolicy)
        -> Result<UtcUnixSeconds, Error>;
}
```

## Data Sources

### Embedded (default, no_std)

The `known()` function returns a compile-time table with all 27 historical leap seconds. This works offline, in embedded systems, and is fully deterministic.

### IERS/NTP Parsing (feature-gated)

> **Not yet implemented** — planned for v0.2. See [ROADMAP.md](ROADMAP.md).

With the `iers_parse` feature, you can parse the standard `leap-seconds.list` file distributed by IERS/NIST/NTP:

```rust
let data = std::fs::read_to_string("leap-seconds.list")?;
let table = LeapSeconds::from_iers_list(&data)?;
```

The file format is the NTP leap-seconds.list format:
- Lines starting with `#` are comments
- `#$` contains the last update timestamp
- `#@` contains the expiration timestamp
- Data lines: `NTP_TIMESTAMP TAI_OFFSET`

### Custom Tables (for testing)

```rust
let table = LeapSeconds::builder()
    .add(UtcUnixSeconds(63_072_000), 10)   // 1972-01-01: TAI−UTC = 10
    .add(UtcUnixSeconds(78_796_800), 11)   // 1972-07-01: TAI−UTC = 11
    // ...
    .build()?;
```

## Errors

```rust
pub enum Error {
    /// The timestamp is outside the range of the table.
    OutOfRange { requested: i64, valid_start: i64, valid_end: i64 },
    /// The leap-second table has expired.
    TableExpired { expires_at: i64 },
    /// The table data is invalid (used during builder validation).
    InvalidTable { detail: &'static str },
}
```

## Negative Leap Seconds

The ITU-R Recommendation TF.460 specification allows *negative* leap seconds
(where the TAI−UTC offset decreases by 1 and the UTC clock skips from `23:59:58`
directly to `00:00:00`, omitting `23:59:59`). No negative leap second has ever
been applied — all 27 historical insertions have been positive.

Nevertheless, the table format and conversion logic must handle them:

- A table entry where the new offset is *less* than the previous offset
  represents a negative leap second.
- `is_during_leap_second()` returns `false` for negative leap seconds (there is
  no extra second; instead a second is removed).
- Roundtrip conversion `utc_to_tai |> tai_to_utc` remains correct: the skipped
  UTC second simply has no pre-image.

This is a correctness requirement, not a likely scenario. The custom table
builder should accept negative entries for testing.

## Guardrails

- No hidden global leap-second state — the table is always an explicit parameter
- Conversions are pure functions: `(table, input) → output`
- No silent fallback behavior (no "assume no more leap seconds") — except the
  explicit opt-in `ExpirationPolicy::Extrapolate` mode, which is loudly
  documented as simulation-only
- No `unsafe` code
- All conversions tested against known reference values at leap-second boundaries
- Negative leap seconds are handled correctly even though none have occurred
