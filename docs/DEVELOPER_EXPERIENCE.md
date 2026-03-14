# Developer Experience

The DX bar for `leap-sec` is **Rails-level readability**: the API should be so small and obvious that a developer can use it correctly without reading the docs.

## The Golden Rule

**You cannot accidentally do a UTC↔TAI conversion without a leap-second table. And using the table should take one line.**

```rust
use leap_sec::prelude::*;

let leaps = LeapSeconds::known();           // one line to get the table
let tai = leaps.utc_to_tai(utc)?;           // one line to convert
```

That's it. Two lines. No configuration, no builder pattern, no ceremony. If this ever becomes more complex, we failed.

## API Readability Standards

### Every method reads as a sentence

```rust
let leaps = LeapSeconds::known();

// ✅ Reads as: "leaps: UTC to TAI"
let tai = leaps.utc_to_tai(utc)?;

// ✅ "leaps: TAI to UTC"
let utc = leaps.tai_to_utc(tai)?;

// ✅ "leaps: UTC to GPST"
let gpst = leaps.utc_to_gpst(utc)?;

// ✅ "leaps: TAI-UTC offset at this UTC instant"
let offset = leaps.tai_utc_offset(utc)?;        // → i32 (e.g., 37)

// ✅ "is this table expired?"
if leaps.is_expired() { /* warn or update */ }
```

### Types prevent mixing

```rust
let utc = UtcUnixSeconds(1_700_000_000);    // explicitly UTC
let tai = TaiSeconds(1_700_000_037);         // explicitly TAI

// ✅ Correct: UTC goes into utc_to_tai
let result = leaps.utc_to_tai(utc)?;

// ❌ Compile error: TaiSeconds cannot be passed to utc_to_tai
// let wrong = leaps.utc_to_tai(tai);
// error: expected `UtcUnixSeconds`, found `TaiSeconds`
```

You literally cannot pass the wrong type. The function signature prevents it.

### Errors explain what happened and what to do

```rust
let pre_1972 = UtcUnixSeconds(0);  // Before the modern UTC system
let result = leaps.utc_to_tai(pre_1972);

// Err(Error::OutOfRange {
//     requested: 0,
//     valid_start: 63072000,   // 1972-01-01
//     valid_end: 1483228800,   // 2017-01-01
// })
```

Errors are **structured** (matchable in production systems) and **helpful** (the fields tell you exactly what the valid range is).

### Display is useful

```rust
let utc = UtcUnixSeconds(1_700_000_000);
println!("{utc}");  // "1700000000 UTC"

let tai = TaiSeconds(1_700_000_037);
println!("{tai}");  // "1700000037 TAI"

// The scale label is always visible — you cannot accidentally print
// a TAI value and mistake it for UTC
```

## The First 5 Minutes

```rust
use leap_sec::prelude::*;

fn main() -> Result<(), Error> {
    // Get the built-in leap-second table
    let leaps = LeapSeconds::known();

    // Convert UTC to TAI
    let utc = UtcUnixSeconds(1_700_000_000);
    let tai = leaps.utc_to_tai(utc)?;

    // Convert back
    let roundtrip = leaps.tai_to_utc(tai)?;
    assert_eq!(roundtrip, utc);

    // Check the current offset
    let offset = leaps.tai_utc_offset(utc)?;
    assert_eq!(offset, 37); // TAI is 37 seconds ahead of UTC since 2017-01-01

    Ok(())
}
```

Zero configuration. Zero ceremony. It just works.

## Sub-Second Precision

When you need nanosecond resolution (GNSS receivers, spacecraft telemetry),
use the `Nanos` variants. Conversions work identically — the leap-second
offset is applied in whole seconds and the fractional part is preserved:

```rust
use leap_sec::prelude::*;

fn main() -> Result<(), Error> {
    let leaps = LeapSeconds::known();

    // A UTC timestamp with sub-second precision
    let utc_ns = UtcUnixNanos(1_700_000_000_500_000_000); // 0.5s past the epoch second

    let tai_ns = leaps.utc_to_tai_nanos(utc_ns)?;
    // TAI = UTC + 37s offset → nanoseconds shifted by 37_000_000_000
    assert_eq!(tai_ns, TaiNanos(1_700_000_037_500_000_000));

    // Lossless promotion from seconds to nanos
    let utc_sec = UtcUnixSeconds(1_700_000_000);
    let utc_ns_from_sec: UtcUnixNanos = utc_sec.into();
    assert_eq!(utc_ns_from_sec, UtcUnixNanos(1_700_000_000_000_000_000));

    // Truncating back to seconds
    let back_to_sec = utc_ns.to_seconds_floor();
    assert_eq!(back_to_sec, UtcUnixSeconds(1_700_000_000));

    Ok(())
}
```

## Checking if a Timestamp Falls During a Leap Second

The 61st second (`23:59:60`) is ambiguous in POSIX timestamps. The library
lets you detect it:

```rust
use leap_sec::prelude::*;

let leaps = LeapSeconds::known();

// 2016-12-31T23:59:60 UTC — the leap second inserted before 2017-01-01.
// In POSIX time this folds to the same value as 2017-01-01T00:00:00.
let ambiguous = UtcUnixSeconds(1_483_228_800);

if leaps.is_during_leap_second(ambiguous) {
    println!("This timestamp is during a leap-second insertion.");
    println!("The UTC wall clock reads 23:59:60.");
}

// Normal timestamps return false:
let normal = UtcUnixSeconds(1_700_000_000);
assert!(!leaps.is_during_leap_second(normal));
```

## Table Inspection

Check the table's range, the most recent entry, or expiration:

```rust
use leap_sec::prelude::*;

let leaps = LeapSeconds::known();

// Valid range
let (start, end) = leaps.valid_range();
// start = 63072000 (1972-01-01), end = 1483228800 (2017-01-01)

// Most recent leap second
let (date, offset) = leaps.latest_entry();
// date = 1483228800 (2017-01-01), offset = 37

// The built-in table has no expiration
assert_eq!(leaps.expires_at(), None);
assert!(!leaps.is_expired());
```

## Custom Tables for Testing

Build synthetic tables with the builder (requires `std` feature):

```rust
use leap_sec::prelude::*;

let table = LeapSeconds::builder()
    .add(UtcUnixSeconds(1000), 10)
    .add(UtcUnixSeconds(2000), 11)
    .expires_at(UtcUnixSeconds(5000))
    .build()
    .unwrap();

let tai = table.utc_to_tai(UtcUnixSeconds(1500)).unwrap();
assert_eq!(tai, TaiSeconds(1510));

assert_eq!(table.expires_at(), Some(UtcUnixSeconds(5000)));
```

## TAI to/from GPST — Direct Path

TAI and GPS Time differ by a constant 19 seconds. No table is needed:

```rust
use leap_sec::prelude::*;

let tai = TaiSeconds(1_700_000_037);
let gpst = tai_to_gpst(tai);
assert_eq!(gpst, GpstSeconds(1_700_000_018)); // TAI − 19

let back = gpst_to_tai(gpst);
assert_eq!(back, tai);
```

These are free functions, not methods on `LeapSeconds`, because the 19-second
offset is a physical constant — always valid, no table required.

## What Gets Exported in the Prelude

```rust
pub mod prelude {
    pub use crate::{
        LeapSeconds,
        UtcUnixSeconds, TaiSeconds, GpstSeconds,
        UtcUnixNanos, TaiNanos, GpstNanos,
        tai_to_gpst, gpst_to_tai,
        tai_to_gpst_nanos, gpst_to_tai_nanos,
        Error,
        LeapSecondsBuilder,  // behind #[cfg(feature = "std")]
    };
}
```

Core types, nanosecond variants, the direct TAI/GPST helpers (seconds and nanos),
the error type, and the builder. That is the whole public API surface for v0.1.

## Naming Conventions

| Pattern | Example | Reads as |
|---------|---------|----------|
| `LeapSeconds::known()` | `let leaps = LeapSeconds::known()` | "known leap seconds" |
| `.utc_to_tai(utc)` | `leaps.utc_to_tai(utc)` | "UTC to TAI" |
| `.utc_to_tai_nanos(utc)` | `leaps.utc_to_tai_nanos(utc)` | "UTC to TAI (nanos)" |
| `.tai_to_utc(tai)` | `leaps.tai_to_utc(tai)` | "TAI to UTC" |
| `.utc_to_gpst(utc)` | `leaps.utc_to_gpst(utc)` | "UTC to GPST" |
| `.gpst_to_utc(gpst)` | `leaps.gpst_to_utc(gpst)` | "GPST to UTC" |
| `.tai_utc_offset(utc)` | `leaps.tai_utc_offset(utc)` | "TAI-UTC offset at this time" |
| `.tai_utc_offset_at_tai(tai)` | `leaps.tai_utc_offset_at_tai(tai)` | "TAI-UTC offset at this TAI" |
| `.is_expired()` | `leaps.is_expired()` | "is the table expired?" |
| `.is_during_leap_second(utc)` | `leaps.is_during_leap_second(utc)` | "is this during a leap second?" |
| `.valid_range()` | `leaps.valid_range()` | "valid range of the table" |
| `.expires_at()` | `leaps.expires_at()` | "when does this table expire?" |
| `.latest_entry()` | `leaps.latest_entry()` | "most recent leap second entry" |
| `tai_to_gpst(tai)` | `tai_to_gpst(tai)` | "TAI to GPST (constant offset)" |
| `gpst_to_tai(gpst)` | `gpst_to_tai(gpst)` | "GPST to TAI (constant offset)" |

No abbreviations. No jargon in method names. If you can read English, you can read the API.

## Testing Strategy

- **Behave specs**: every conversion reads like a requirement:
  ```
  given a known leap-second table
  when converting UTC 2017-01-01T00:00:00 to TAI
  then the result is UTC + 37 seconds
  ```
- **Boundary tests**: test the exact second before, during, and after every leap-second insertion
- **Roundtrip invariants**: `utc_to_tai(x) |> tai_to_utc == x` for all valid inputs
- **Table fixtures**: custom tables for testing edge cases (single leap second, expired table, empty table)

## Feature Flags

| Flag | What it enables |
|------|----------------|
| `std` (default) | `LeapSecondsBuilder`, `std::error::Error` impl |

To use in `no_std`, disable default features:

```toml
[dependencies]
leap-sec = { version = "0.1", default-features = false }
```
