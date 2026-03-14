# `LeapSeconds::utc_to_gpst_nanos`

```rust
pub fn utc_to_gpst_nanos(&self, utc: UtcUnixNanos) -> Result<GpstNanos, Error>
```

Convert UTC Unix nanoseconds to GPST nanoseconds.

## Logic

Composes [`utc_to_tai_nanos`](utc_to_tai_nanos.md) then
[`tai_to_gpst_nanos`](tai_to_gpst_nanos.md).

## Errors

Returns [`Error::OutOfRange`](Error.md) if the timestamp is before 1972-01-01.

## Example

```rust
use leap_sec::prelude::*;

let leaps = LeapSeconds::known();

let utc = UtcUnixNanos(1_700_000_000_250_000_000);
let gpst = leaps.utc_to_gpst_nanos(utc).unwrap();
assert_eq!(gpst, GpstNanos(1_700_000_018_250_000_000));
```

## See Also

- [`gpst_to_utc_nanos`](gpst_to_utc_nanos.md) — the inverse
- [`utc_to_gpst`](utc_to_gpst.md) — seconds variant
