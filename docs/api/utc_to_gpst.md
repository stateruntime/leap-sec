# `LeapSeconds::utc_to_gpst`

```rust
pub fn utc_to_gpst(&self, utc: UtcUnixSeconds) -> Result<GpstSeconds, Error>
```

Convert a UTC Unix timestamp to GPS Time.

## Logic

Composes [`utc_to_tai`](utc_to_tai.md) then [`tai_to_gpst`](tai_to_gpst.md)
(subtracts 19 from the TAI result).

## Errors

Returns [`Error::OutOfRange`](Error.md) if `utc` is before the table's range.

## Example

```rust
use leap_sec::prelude::*;

let leaps = LeapSeconds::known();

// TAI = 1_700_000_037, GPST = TAI - 19 = 1_700_000_018
let gpst = leaps.utc_to_gpst(UtcUnixSeconds(1_700_000_000)).unwrap();
assert_eq!(gpst, GpstSeconds(1_700_000_018));
```

## See Also

- [`gpst_to_utc`](gpst_to_utc.md) — the inverse
- [`utc_to_gpst_nanos`](utc_to_gpst_nanos.md) — nanosecond variant
