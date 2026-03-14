# `LeapSeconds::utc_to_tai`

```rust
pub fn utc_to_tai(&self, utc: UtcUnixSeconds) -> Result<TaiSeconds, Error>
```

Convert a UTC Unix timestamp to TAI seconds.

## Logic

Binary-searches the table for the largest entry whose timestamp is ≤ `utc`,
then returns `utc + offset`.

## Errors

Returns [`Error::OutOfRange`](Error.md) if `utc` is before the first entry
(1972-01-01 for the built-in table).

## Example

```rust
use leap_sec::prelude::*;

let leaps = LeapSeconds::known();

// 2023-11-14 — offset is 37
let tai = leaps.utc_to_tai(UtcUnixSeconds(1_700_000_000)).unwrap();
assert_eq!(tai, TaiSeconds(1_700_000_037));

// Before 1972 — error
let err = leaps.utc_to_tai(UtcUnixSeconds(0));
assert!(err.is_err());
```

## See Also

- [`tai_to_utc`](tai_to_utc.md) — the inverse
- [`utc_to_tai_nanos`](utc_to_tai_nanos.md) — nanosecond variant
