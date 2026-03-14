# `LeapSeconds::gpst_to_utc`

```rust
pub fn gpst_to_utc(&self, gpst: GpstSeconds) -> Result<UtcUnixSeconds, Error>
```

Convert GPS Time to a UTC Unix timestamp.

## Logic

Composes [`gpst_to_tai`](gpst_to_tai.md) (adds 19) then
[`tai_to_utc`](tai_to_utc.md).

## Errors

Returns [`Error::OutOfRange`](Error.md) if the resulting TAI is before the
table's range.

## Example

```rust
use leap_sec::prelude::*;

let leaps = LeapSeconds::known();

let utc = leaps.gpst_to_utc(GpstSeconds(1_700_000_018)).unwrap();
assert_eq!(utc, UtcUnixSeconds(1_700_000_000));
```

## See Also

- [`utc_to_gpst`](utc_to_gpst.md) — the inverse
- [`gpst_to_utc_nanos`](gpst_to_utc_nanos.md) — nanosecond variant
