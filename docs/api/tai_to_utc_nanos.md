# `LeapSeconds::tai_to_utc_nanos`

```rust
pub fn tai_to_utc_nanos(&self, tai: TaiNanos) -> Result<UtcUnixNanos, Error>
```

Convert TAI nanoseconds to UTC Unix nanoseconds.

## Errors

Returns [`Error::OutOfRange`](Error.md) if the timestamp is before the table's
TAI range.

## Example

```rust
use leap_sec::prelude::*;

let leaps = LeapSeconds::known();

let tai = TaiNanos(1_700_000_037_500_000_000);
let utc = leaps.tai_to_utc_nanos(tai).unwrap();
assert_eq!(utc, UtcUnixNanos(1_700_000_000_500_000_000));
```

## See Also

- [`utc_to_tai_nanos`](utc_to_tai_nanos.md) — the inverse
- [`tai_to_utc`](tai_to_utc.md) — seconds variant
