# `LeapSeconds::tai_utc_offset_at_tai`

```rust
pub fn tai_utc_offset_at_tai(&self, tai: TaiSeconds) -> Result<i32, Error>
```

Get the TAI−UTC offset at a given TAI instant.

Useful when you already have a TAI timestamp and want to know the offset
without first converting to UTC.

## Errors

Returns [`Error::OutOfRange`](Error.md) if `tai` is before the first TAI
boundary in the table.

## Example

```rust
use leap_sec::prelude::*;

let leaps = LeapSeconds::known();

assert_eq!(leaps.tai_utc_offset_at_tai(TaiSeconds(1_700_000_037)).unwrap(), 37);
```

## See Also

- [`tai_utc_offset`](tai_utc_offset.md) — same query in UTC space
