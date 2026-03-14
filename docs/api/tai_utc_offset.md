# `LeapSeconds::tai_utc_offset`

```rust
pub fn tai_utc_offset(&self, utc: UtcUnixSeconds) -> Result<i32, Error>
```

Get the TAI−UTC offset at a given UTC instant.

## Returns

The number of whole seconds that TAI is ahead of UTC at this moment.
For example, `37` means TAI = UTC + 37s.

## Errors

Returns [`Error::OutOfRange`](Error.md) if `utc` is before 1972-01-01.

## Example

```rust
use leap_sec::prelude::*;

let leaps = LeapSeconds::known();

// In 1972: offset was 10
assert_eq!(leaps.tai_utc_offset(UtcUnixSeconds(63_072_000)).unwrap(), 10);

// After 2017-01-01: offset is 37
assert_eq!(leaps.tai_utc_offset(UtcUnixSeconds(1_700_000_000)).unwrap(), 37);
```

## See Also

- [`tai_utc_offset_at_tai`](tai_utc_offset_at_tai.md) — same query in TAI space
