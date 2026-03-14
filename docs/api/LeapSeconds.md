# `LeapSeconds`

An immutable leap-second schedule.

## Overview

`LeapSeconds` holds a sorted list of `(utc_timestamp, tai_minus_utc)` pairs and
provides all conversion methods between UTC, TAI, and GPST.

Use [`known()`](known.md) to get the built-in table. Use
[`LeapSecondsBuilder`](LeapSecondsBuilder.md) to construct custom tables for
testing.

## Example

```rust
use leap_sec::prelude::*;

let leaps = LeapSeconds::known();

let utc = UtcUnixSeconds(1_700_000_000);
let tai = leaps.utc_to_tai(utc).unwrap();
assert_eq!(tai, TaiSeconds(1_700_000_037));
```

## Methods

### Constructors

- [`known()`](known.md) — built-in historical table
- [`builder()`](LeapSecondsBuilder.md) — custom table builder (requires `std`)

### Conversions

- [`utc_to_tai`](utc_to_tai.md) / [`tai_to_utc`](tai_to_utc.md)
- [`utc_to_gpst`](utc_to_gpst.md) / [`gpst_to_utc`](gpst_to_utc.md)
- [`utc_to_tai_nanos`](utc_to_tai_nanos.md) / [`tai_to_utc_nanos`](tai_to_utc_nanos.md)
- [`utc_to_gpst_nanos`](utc_to_gpst_nanos.md) / [`gpst_to_utc_nanos`](gpst_to_utc_nanos.md)

### Offsets & Detection

- [`tai_utc_offset`](tai_utc_offset.md) / [`tai_utc_offset_at_tai`](tai_utc_offset_at_tai.md)
- [`is_during_leap_second`](is_during_leap_second.md)

### Inspection

- [`valid_range`](valid_range.md), [`is_expired`](is_expired.md), [`expires_at`](expires_at.md), [`latest_entry`](latest_entry.md)
