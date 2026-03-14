# leap-sec

[![Crates.io](https://img.shields.io/crates/v/leap-sec.svg)](https://crates.io/crates/leap-sec)
[![Documentation](https://docs.rs/leap-sec/badge.svg)](https://docs.rs/leap-sec)
[![CI](https://github.com/stateruntime/leap-sec/actions/workflows/ci.yml/badge.svg)](https://github.com/stateruntime/leap-sec/actions/workflows/ci.yml)
[![License](https://img.shields.io/crates/l/leap-sec.svg)](LICENSE)
[![MSRV](https://img.shields.io/badge/MSRV-1.75-blue.svg)](https://blog.rust-lang.org/)

**Leap-second table and UTC ↔ TAI conversion kernel.**

`leap-sec` does **one thing well**: given a table of leap-second insertions, it converts timestamps between UTC (which has leap seconds) and continuous time scales (TAI/GPS, which don't).

For space systems, GNSS receivers, satellite operations, and any software that cannot silently ignore leap seconds.

---

## Install

```toml
[dependencies]
leap-sec = "0.1"
```

## Start Fast

```rust
use leap_sec::prelude::*;

fn main() -> Result<(), Error> {
    let leaps = LeapSeconds::known();

    // Convert UTC to TAI
    let utc = UtcUnixSeconds(1_700_000_000);   // 2023-11-14 22:13:20 UTC
    let tai = leaps.utc_to_tai(utc)?;          // TAI = UTC + 37 seconds
    assert_eq!(tai, TaiSeconds(1_700_000_037));

    // Convert back — exact roundtrip
    let back = leaps.tai_to_utc(tai)?;
    assert_eq!(back, utc);

    // Check the current offset
    let offset = leaps.tai_utc_offset(utc)?;
    assert_eq!(offset, 37);

    Ok(())
}
```

That's it. Two lines to get a table and convert. No configuration, no builder pattern, no ceremony.

## Sub-Second Precision

When you need nanosecond resolution (GNSS receivers, spacecraft telemetry),
use the `Nanos` variants:

```rust
use leap_sec::prelude::*;

fn main() -> Result<(), Error> {
    let leaps = LeapSeconds::known();

    let utc_ns = UtcUnixNanos(1_700_000_000_500_000_000); // 0.5s past the second
    let tai_ns = leaps.utc_to_tai_nanos(utc_ns)?;
    assert_eq!(tai_ns, TaiNanos(1_700_000_037_500_000_000));

    // Lossless promotion from seconds to nanos
    let utc_sec = UtcUnixSeconds(1_700_000_000);
    let promoted: UtcUnixNanos = utc_sec.into();
    assert_eq!(promoted, UtcUnixNanos(1_700_000_000_000_000_000));

    Ok(())
}
```

## TAI ↔ GPST — No Table Needed

TAI and GPS Time differ by a constant 19 seconds. These are free functions,
always valid:

```rust
use leap_sec::prelude::*;

let tai = TaiSeconds(1_700_000_037);
let gpst = tai_to_gpst(tai);
assert_eq!(gpst, GpstSeconds(1_700_000_018)); // TAI − 19

let back = gpst_to_tai(gpst);
assert_eq!(back, tai);
```

## What Is a Leap Second?

The Earth's rotation is gradually slowing down. To keep civil time (UTC) in sync with the Earth's actual position, the international timekeeping authority (IERS) occasionally inserts an extra second — a **leap second** — at the end of June 30 or December 31.

When a leap second is inserted, the clock goes: `23:59:59` → `23:59:60` → `00:00:00`.

**Since 1972, leap seconds have been inserted 27 times** (most recently on **2016-12-31**, effective **2017-01-01**). Since that insertion, the offset has been:

- **TAI − UTC = 37 seconds**
- **GPS − UTC = 18 seconds** (because **TAI − GPS = 19 seconds** by definition)

These offsets change only when a leap second is inserted.

```
Timeline showing a leap second insertion (Dec 31, 2016):

TAI:  ...  2017-01-01 00:00:35  │  2017-01-01 00:00:36  │  2017-01-01 00:00:37  ...
                                │                       │
UTC:  ...  2016-12-31 23:59:59  │  2016-12-31 23:59:60  │  2017-01-01 00:00:00  ...
                                │       ↑               │
                                │  This second only      │
                                │  exists in UTC         │
```

## Why Does This Matter for Software?

Most software uses **Unix time** — a seconds counter since 1970-01-01 that pretends leap seconds don't exist. This works fine for everyday applications, but causes real bugs in systems that need **accurate, continuous time**:

### Bug 1: Events appear out of order
```
Telemetry timestamp (UTC):     23:59:60.500  →  Unix time: ???
Telemetry timestamp (UTC):     00:00:00.200  →  Unix time: 1483228800.2

The leap second (23:59:60) can't be represented in Unix time.
Some systems map it to 23:59:59 (duplicate!) or 00:00:00 (out of order!).
Result: telemetry events appear to arrive before they were sent.
```

### Bug 2: Two systems silently disagree
```
System A uses TAI:    elapsed = 1,483,228,837 seconds
System B uses UTC:    elapsed = 1,483,228,800 seconds
Difference:           37 seconds (the accumulated leap seconds)

If nobody tracks which system uses which time scale,
these timestamps look "close enough" but are actually
referencing different instants — or the same instant
with a 37-second naming disagreement.
```

### Bug 3: "Why are these events 18 seconds apart?"
```
In 2005, GPS time was 13 seconds ahead of UTC.
Since 2017-01-01, GPS time has been 18 seconds ahead of UTC.
A ground system that doesn't update its leap-second table
will miscorrelate GPS-timestamped data by the wrong offset.
```

## Real-World Scenarios

### Ground Station Correlating GPS Data

A ground station receives GPS-timestamped satellite observations and must merge
them with UTC-timestamped ground truth.

```rust
let leaps = LeapSeconds::known();
let gps_obs = GpstSeconds(receiver_timestamp);
let utc_obs = leaps.gpst_to_utc(gps_obs)?;
// Both timelines now in UTC for correlation
```

### Archival System Spanning Multiple Leap Seconds

A climate data archive holds records from 1975 to 2024. Converting to a
continuous TAI timeline eliminates leap-second discontinuities.

```rust
let leaps = LeapSeconds::known();
for record in &archive {
    let tai = leaps.utc_to_tai(record.utc)?;
    // Correct offset (14..37) selected automatically per era
}
```

### Checking Table Freshness Before a Mission

```rust
let leaps = LeapSeconds::known();
let (_, offset) = leaps.latest_entry();
println!("Current TAI-UTC offset: {offset}s");
println!("Table expired: {}", leaps.is_expired());
```

## What It Does

- **Embeds a known leap-second table** — works offline, works in `no_std`, deterministic
- **Converts UTC ↔ TAI** — step-based (the real, correct conversion)
- **Converts UTC ↔ GPST** — TAI offset minus 19 seconds
- **Nanosecond precision** — `UtcUnixNanos`, `TaiNanos`, `GpstNanos` with lossless conversion
- **Leap-second detection** — `is_during_leap_second()` flags the ambiguous 61st second
- **Type safety** — six newtypes prevent accidentally mixing time scales
- **Explicit errors** — if the table doesn't cover your timestamp, you get `OutOfRange`, not a wrong answer
- **Custom tables** — `LeapSecondsBuilder` for testing with synthetic schedules

## What It Does NOT Do

- **Full datetime/calendar/timezone support** — use `jiff`, `time`, or `chrono`
- **NTP/PTP clock sync** — this is a conversion function, not a clock
- **Auto-download tables** — networking is opt-in, not default
- **IERS file parsing** — planned for v0.2

## What Kind of Timestamp Does It Accept?

`leap-sec` is intentionally **not** a full datetime library. Its core API works with *seconds counters*:

- `UtcUnixSeconds`: a Unix/POSIX-style seconds counter interpreted in the UTC scale
  (note: POSIX time does **not** represent the leap second label `23:59:60`)
- `TaiSeconds` / `GpstSeconds`: continuous seconds counters (no leap seconds)
- `UtcUnixNanos` / `TaiNanos` / `GpstNanos`: nanosecond-precision variants using `i128`

If you need calendar parsing/formatting (`YYYY-MM-DD...`), use `jiff`, `time`, or `chrono` to get the seconds count, then pass it to `leap-sec`.

## Features

| Feature | Default | Description |
|---------|---------|-------------|
| `std` | Yes | Standard library support, `LeapSecondsBuilder`, `std::error::Error` impl |

## `no_std` Support

Disable the default `std` feature for embedded and `no_std` environments:

```toml
[dependencies]
leap-sec = { version = "0.1", default-features = false }
```

Everything works except `LeapSecondsBuilder` (which requires heap allocation).

## The 2030s UTC Change (leap seconds likely to end)

In 2022, the 27th CGPM adopted **Resolution 4** on the future of UTC: the plan is to increase the
maximum allowed value of `|UT1−UTC|` (currently kept within ±0.9s using leap seconds) at a date to be
decided, with key decisions targeted **by or before 2035**.

In practice, this is widely understood as the path to **ending leap seconds** (or at least making them
unnecessary) for a long time.

For software, the future is not "leap seconds disappear" — it's:

- All 27 historical leap seconds remain in existing datasets
- Protocols and archives will reference these timestamps for decades
- Systems spanning the transition need both "old UTC with leaps" and "new UTC without leaps"

`leap-sec` is designed for this: the table is explicit data, not hardcoded assumptions. When leap seconds stop, the table simply stops growing — existing conversions remain correct.

## Why Not Just Hardcode the Offset?

Because the offset **changes over time**. It was 10 seconds in 1972 and 37 seconds in 2017. Each leap second insertion changes the offset. And the decision to insert a leap second is made only ~6 months in advance by the IERS, based on astronomical observations.

The only correct approach is a **table** that maps date ranges to offsets. That's what `leap-sec` manages.

## Why Rely On It

- `unsafe` is forbidden by lint configuration
- All 27 leap-second boundaries tested (before, during, after each insertion)
- Roundtrip invariant: `utc_to_tai(x) |> tai_to_utc == x` for all valid inputs
- 110 behave specs + 32 doc-tests, all passing
- Clippy pedantic + nursery enforced with zero warnings
- `no_std` tested in CI
- MSRV 1.75 checked in CI
- Limitations are documented explicitly instead of left implicit
- Security reporting is documented in [SECURITY.md](SECURITY.md)

## Error Handling

```rust
use leap_sec::prelude::*;

fn convert(utc: UtcUnixSeconds) -> Result<TaiSeconds, Error> {
    let leaps = LeapSeconds::known();
    leaps.utc_to_tai(utc)
}

// Pattern-match for production error handling:
match convert(UtcUnixSeconds(0)) {
    Ok(tai) => println!("TAI: {tai}"),
    Err(Error::OutOfRange { valid_start, .. }) => {
        eprintln!("Timestamp before table start ({valid_start})");
    }
    Err(e) => eprintln!("Error: {e}"),
}
```

## Examples

Runnable examples in [`examples/`](examples/):

| Example | Description | Run |
|---------|-------------|-----|
| [`quickstart`](examples/quickstart.rs) | Basic UTC ↔ TAI conversion and roundtrip | `cargo run --example quickstart` |
| [`nanoseconds`](examples/nanoseconds.rs) | Sub-second precision, promotion, floor truncation | `cargo run --example nanoseconds` |
| [`gps_time`](examples/gps_time.rs) | TAI ↔ GPST free functions and UTC → GPST via table | `cargo run --example gps_time` |
| [`leap_second_detection`](examples/leap_second_detection.rs) | Detecting the ambiguous 61st second (23:59:60) | `cargo run --example leap_second_detection` |
| [`table_inspection`](examples/table_inspection.rs) | Inspecting valid range, latest entry, expiration | `cargo run --example table_inspection` |
| [`custom_table`](examples/custom_table.rs) | Building a custom table with `LeapSecondsBuilder` | `cargo run --example custom_table` |

## Documentation

- [API Reference](docs/api/README.md) — per-type and per-method reference pages
- [API docs on docs.rs](https://docs.rs/leap-sec)
- [Design](docs/DESIGN.md) — API shape and conversion semantics
- [Developer Experience](docs/DEVELOPER_EXPERIENCE.md) — DX standards
- [Roadmap](docs/ROADMAP.md) — release plan v0.1 → v1.0
- [Landscape](docs/landscape.md) — ecosystem positioning
- [Vision](docs/VISION.md) — why this exists
- [Audit](docs/AUDIT.md) — competitive analysis

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE).
