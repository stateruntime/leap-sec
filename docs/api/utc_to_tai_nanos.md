# `LeapSeconds::utc_to_tai_nanos`

```rust
pub fn utc_to_tai_nanos(&self, utc: UtcUnixNanos) -> Result<TaiNanos, Error>
```

Convert UTC Unix nanoseconds to TAI nanoseconds.

## Logic

Floors the nanoseconds to whole seconds for the table lookup, then applies the
offset in nanoseconds. Sub-second precision is preserved exactly.

## Errors

Returns [`Error::OutOfRange`](Error.md) if the timestamp is before 1972-01-01.

## Example

```rust
use leap_sec::prelude::*;

let leaps = LeapSeconds::known();

let utc = UtcUnixNanos(1_700_000_000_500_000_000);
let tai = leaps.utc_to_tai_nanos(utc).unwrap();
assert_eq!(tai, TaiNanos(1_700_000_037_500_000_000));
```

## See Also

- [`tai_to_utc_nanos`](tai_to_utc_nanos.md) — the inverse
- [`utc_to_tai`](utc_to_tai.md) — seconds variant
