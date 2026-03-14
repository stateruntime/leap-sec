# `LeapSeconds::gpst_to_utc_nanos`

```rust
pub fn gpst_to_utc_nanos(&self, gpst: GpstNanos) -> Result<UtcUnixNanos, Error>
```

Convert GPST nanoseconds to UTC Unix nanoseconds.

## Logic

Composes [`gpst_to_tai_nanos`](gpst_to_tai_nanos.md) then
[`tai_to_utc_nanos`](tai_to_utc_nanos.md).

## Errors

Returns [`Error::OutOfRange`](Error.md) if the resulting TAI is before the
table's range.

## Example

```rust
use leap_sec::prelude::*;

let leaps = LeapSeconds::known();

let gpst = GpstNanos(1_700_000_018_250_000_000);
let utc = leaps.gpst_to_utc_nanos(gpst).unwrap();
assert_eq!(utc, UtcUnixNanos(1_700_000_000_250_000_000));
```

## See Also

- [`utc_to_gpst_nanos`](utc_to_gpst_nanos.md) — the inverse
- [`gpst_to_utc`](gpst_to_utc.md) — seconds variant
